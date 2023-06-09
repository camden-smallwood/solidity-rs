mod analysis;
mod brownie;
mod hardhat;
mod report;
mod truffle;
mod todo_list;

use report::Report;
use solidity::ast::*;
use std::{cell::RefCell, collections::HashSet, env, fs::File, io, path::PathBuf, rc::Rc};

#[derive(Debug)]
enum OutputFormat {
    PlainText,
    Json,
}

impl TryFrom<&str> for OutputFormat {
    type Error = io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "plaintext" | "plain-text" | "plain_text" => Ok(Self::PlainText),
            "json" => Ok(Self::Json),
            s => Err(io::Error::new(io::ErrorKind::Unsupported, s)),
        }
    }
}

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next().ok_or_else(|| io::Error::from(io::ErrorKind::BrokenPipe))?;

    let mut path: Option<PathBuf> = None;
    let mut should_print_todo_list = false;
    let mut visitor_names: HashSet<String> = HashSet::new();
    let mut contract_name: Option<String> = None;
    let mut output_format = OutputFormat::PlainText;
    let mut contract_paths = vec![];

    for arg in args {
        match arg {
            arg if arg.starts_with("--") => match &arg.as_str()[2..] {
                "todo-list" | "todo_list" => {
                    should_print_todo_list = true;
                }

                s if s.starts_with("contract=") => {
                    if contract_name.is_some() {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Multiple contracts specified: {} {}", path.unwrap().to_string_lossy(), arg)));
                    }
                    
                    contract_name = Some(s.trim_start_matches("contract=").into());
                }

                s if s.starts_with("contract-path=") || s.starts_with("contract_path=") => {
                    contract_paths.push(PathBuf::from(&arg.as_str()[16..]));
                }

                s if s.starts_with("output-format=") || s.starts_with("output_format=") => {
                    output_format = OutputFormat::try_from(&arg.as_str()[16..])?;
                }

                s if analysis::VISITOR_TYPES.iter().any(|visitor| visitor.0 == s) => {
                    if !visitor_names.contains(s) {
                        visitor_names.insert(s.into());
                    }
                }

                _ => {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid argument: {}", arg)));
                }
            }

            _ => {
                if path.is_some() {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Multiple paths specified: {} {}", path.unwrap().to_string_lossy(), arg)));
                }

                path = Some(PathBuf::from(arg));
            }
        }
    }

    let mut source_units: Vec<SourceUnit> = vec![];

    if contract_paths.is_empty() {
        let path = match path {
            Some(path) => path,
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path not supplied"))
        };

        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, path.to_string_lossy()))
        }
        
        let brownie_config_path = path.join("brownie-config.yaml");
        let hardhat_config_js_path = path.join("hardhat.config.js");
        let hardhat_config_ts_path = path.join("hardhat.config.ts");
        let truffle_config_path = path.join("truffle-config.js");

        if brownie_config_path.is_file() {
            //
            // TODO: load the brownie config and get the actual build paths
            //

            let build_paths = &[
                path.join("build").join("contracts"),
                path.join("build").join("interfaces"),
            ];

            for build_path in build_paths {
                if !build_path.exists() || !build_path.is_dir() {
                    todo!("brownie project not compiled")
                }

                for path in std::fs::read_dir(build_path)? {
                    let path = path?.path();

                    if !path.is_file() || !path.extension().map(|extension| extension == "json").unwrap_or(false) {
                        continue;
                    }

                    let file: brownie::File = simd_json::from_reader(File::open(path)?)?;

                    if let Some(mut source_unit) = file.ast {
                        if let Some(contract_name) = contract_name.as_deref() {
                            if !source_unit.contract_definitions().iter().any(|c| c.name == contract_name) {
                                continue;
                            }
                        }

                        if !source_units.iter().any(|existing_source_unit| existing_source_unit.absolute_path == source_unit.absolute_path) {
                            source_unit.source = file.source.clone();
                            source_units.push(source_unit);
                        }
                    }
                }
            }
        } else if hardhat_config_js_path.is_file() || hardhat_config_ts_path.is_file() {
            let build_path = path.join("artifacts").join("build-info");

            if !build_path.exists() || !build_path.is_dir() {
                todo!("hardhat project not compiled")
            }

            let console_path = PathBuf::new()
                .join("hardhat")
                .join("console.sol")
                .to_string_lossy()
                .to_string();

            for path in std::fs::read_dir(build_path)? {
                let path = path?.path();

                if !path.is_file() || !path.extension().map(|extension| extension == "json").unwrap_or(false) {
                    continue;
                }

                let file: hardhat::File = simd_json::from_reader(File::open(path)?)?;

                for (source_path, source) in file.output.sources {
                    let mut source_unit = source.ast;

                    if source_unit.absolute_path.as_deref().unwrap_or("").ends_with(console_path.as_str()) {
                        continue;
                    }
                    
                    if let Some(contract_name) = contract_name.as_deref() {
                        if !source_unit.contract_definitions().iter().any(|c| c.name == contract_name) {
                            continue;
                        }
                    }

                    if !source_units.iter().any(|existing_source_unit| existing_source_unit.absolute_path == source_unit.absolute_path) {
                        if let Some(source) = file.input.sources.get(&source_path) {
                            source_unit.source = Some(source.content.clone());
                            source_units.push(source_unit);
                        }
                    }
                }
            }
        } else if truffle_config_path.is_file() {
            let build_path = path.join("build").join("contracts");

            if !build_path.exists() || !build_path.is_dir() {
                todo!("truffle project not compiled")
            }

            let migrations_path = PathBuf::new()
                .join("contracts")
                .join("Migrations.sol")
                .to_string_lossy()
                .to_string();

            for path in std::fs::read_dir(build_path)? {
                let path = path?.path();

                if !path.is_file() || !path.extension().map(|extension| extension == "json").unwrap_or(false) {
                    continue;
                }

                let file: truffle::File = simd_json::from_reader(File::open(path)?)?;

                if let Some(mut source_unit) = file.ast {
                    if source_unit.absolute_path.as_deref().unwrap_or("").ends_with(migrations_path.as_str()) {
                        continue;
                    }
                    
                    if let Some(contract_name) = contract_name.as_deref() {
                        if !source_unit.contract_definitions().iter().any(|c| c.name == contract_name) {
                            continue;
                        }
                    }

                    if !source_units.iter().any(|existing_source_unit| existing_source_unit.absolute_path == source_unit.absolute_path) {
                        source_unit.source = file.source.clone();
                        source_units.push(source_unit);
                    }
                }
            }
        } else {
            unimplemented!("no supported project configuration found")
        }
    } else {
        let mut file_no = 0;

        for contract_path in contract_paths {
            let src = std::fs::read_to_string(contract_path.clone())?;

            let (source_unit, comments) = solang_parser::parse(src.as_str(), file_no)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse contract \"{}\"", contract_path.to_string_lossy())))?;

            let mut builder = AstBuilder::default();
            let mut source_unit = builder.build_source_unit(&source_unit);

            let mut license = None;

            for comment in comments.iter() {
                if let solang_parser::pt::Comment::Line(_, text) = comment {
                    let text = text.trim_start_matches("//").trim_start_matches(' ');
                    if text.starts_with("SPDX-License-Identifier:") {
                        license = Some(text.trim_start_matches("SPDX-License-Identifier:").trim_start_matches(' ').to_string());
                    }
                }
            }

            source_unit.absolute_path = Some(contract_path.to_string_lossy().to_string());
            source_unit.source = Some(src);
            source_unit.license = license;

            source_units.push(source_unit);

            file_no += 1;
        }
    }

    source_units.sort_by(|lhs, rhs| {
        let lhs = lhs.absolute_path.as_deref().unwrap_or("");
        let rhs = rhs.absolute_path.as_deref().unwrap_or("");
        lhs.cmp(rhs)
    });

    if should_print_todo_list {
        todo_list::print(source_units.as_slice());
    }

    let report = Rc::new(RefCell::new(Report::default()));
    let mut visitors: Vec<Box<dyn AstVisitor>> = vec![];

    for &(visitor_name, create_visitor) in analysis::VISITOR_TYPES {
        if visitor_names.is_empty() || visitor_names.contains(visitor_name) {
            visitors.push(create_visitor(report.clone()));
        }
    }

    let mut data = AstVisitorData {
        analyzed_paths: HashSet::new(),
        visitors
    };

    for source_unit in source_units.iter() {
        //
        // Skip node_modules imports
        //

        if source_unit.absolute_path.as_deref().unwrap_or("").starts_with('@') {
            continue;
        }

        //
        // Don't analyze the same source unit multiple times
        //

        if let Some(path) = source_unit.absolute_path.as_ref() {
            if data.analyzed_paths.contains(path) {
                continue;
            }

            data.analyzed_paths.insert(path.clone());
        }

        //
        // Visit the source unit
        //

        let mut context = SourceUnitContext {
            source_units: source_units.as_slice(),
            current_source_unit: source_unit
        };

        data.visit_source_unit(&mut context)?;
        data.leave_source_unit(&mut context)?;
    }

    //
    // Display the report in the desired format
    //

    report.borrow_mut().sort_entries();

    match output_format {
        OutputFormat::PlainText => {
            println!("{}", report.borrow());
        }

        OutputFormat::Json => {
            println!("{}", simd_json::to_string(&report.borrow().clone()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?);
        }
    }

    Ok(())
}
