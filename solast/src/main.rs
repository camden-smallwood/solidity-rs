use std::{env, fs::File, io, path::PathBuf};

pub mod analysis;
pub mod truffle;

fn main() -> io::Result<()> {
    let mut args = env::args();
    let _ = args.next().unwrap();

    let mut path: Option<PathBuf> = None;
    let mut todo_list = false;

    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None => break
        };

        if arg.starts_with("--") {
            match &arg.as_str()[2..] {
                "todo-list" => {
                    todo_list = true;
                }

                _ => {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid argument: {}", arg)));
                }
            }
        } else {
            if path.is_some() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Multiple paths specified: {} {}", path.unwrap().to_string_lossy(), arg)));
            }

            path = Some(PathBuf::from(arg));
        }
    }

    let path = match path {
        Some(path) => path,
        None => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path not supplied"));
        }
    };

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, path.to_string_lossy()))
    }
    
    let mut source_units: Vec<solidity::ast::SourceUnit> = vec![];

    let truffle_config_path = path.join("truffle-config.js");

    if truffle_config_path.is_file() && truffle_config_path.exists() {
        let build_path = path.join("build").join("contracts");

        if !build_path.exists() || !build_path.is_dir() {
            todo!("truffle project not compiled")
        }

        for path in std::fs::read_dir(build_path)? {
            let path = path?.path();

            if !path.exists() || !path.is_file() || !path.extension().map(|extension| extension == "json").unwrap_or(false) {
                continue;
            }
            
            let file: truffle::File = simd_json::from_reader(File::open(path)?)?;

            if let Some(source_unit) = file.ast {
                source_units.push(source_unit);
            }
        }
    } else {
        todo!("truffle config not found; implement support for other project types")
    }

    if todo_list {
        for source_unit in source_units.iter() {
            for contract_definition in source_unit.contract_definitions() {
                if let solidity::ast::ContractKind::Library | solidity::ast::ContractKind::Interface = contract_definition.kind {
                    continue;
                }

                print!("### `");

                if contract_definition.is_abstract.unwrap_or(false) {
                    print!("abstract ");
                }

                print!("{} ", contract_definition.kind);

                println!("{}`:", contract_definition.name);

                for function_definition in contract_definition.function_definitions() {
                    if function_definition.body.is_some() {
                        print!("- [ ] `{}", function_definition.kind);

                        if function_definition.kind != solidity::ast::FunctionKind::Constructor {
                            print!(" {}", function_definition.name);
                        }

                        print!("{}", function_definition.parameters);

                        print!(" {}", function_definition.visibility);

                        for modifier in function_definition.modifiers.iter() {
                            print!(" {}", modifier);
                        }

                        if !function_definition.return_parameters.parameters.is_empty() {
                            print!(" returns {}", function_definition.return_parameters);
                        }

                        println!("`");
                    }
                }

                println!();
            }
        }

        println!();
        println!("----------");
        println!();
    }

    let call_graph = analysis::CallGraph::build(source_units.as_slice())?;

    let mut walker = analysis::AstWalker {
        visitors: vec![
            Box::new(analysis::SourceUnitVisitor::new(source_units.as_slice())),
            Box::new(analysis::NoSpdxIdentifierVisitor),
            Box::new(analysis::FloatingSolidityVersionVisitor),
            Box::new(analysis::NodeModulesImportsVisitor),
            Box::new(analysis::AbstractContractsVisitor),
            Box::new(analysis::LargeLiteralsVisitor::default()),
            Box::new(analysis::RedundantGetterFunctionVisitor::new(source_units.as_slice())),
            Box::new(analysis::RequireWithoutMessageVisitor::new(source_units.as_slice())),
            Box::new(analysis::StateVariableShadowingVisitor::new(source_units.as_slice())),
            Box::new(analysis::ExplicitVariableReturnVisitor::default()),
            Box::new(analysis::UnusedReturnVisitor::new(source_units.as_slice())),
            Box::new(analysis::StorageArrayLoopVisitor::new(source_units.as_slice())),
            Box::new(analysis::ExternalCallsInLoopVisitor::new(source_units.as_slice(), &call_graph)),
            Box::new(analysis::CheckEffectsInteractionsVisitor::new(source_units.as_slice(), &call_graph)),
            Box::new(analysis::RawAddressTransferVisitor::new(source_units.as_slice())),
            Box::new(analysis::SafeERC20FunctionsVisitor::new(source_units.as_slice())),
            Box::new(analysis::UncheckedERC20TransferVisitor::new(source_units.as_slice())),
            Box::new(analysis::UnpaidPayableFunctionsVisitor::new(source_units.as_slice())),
            Box::new(analysis::DivideBeforeMultiplyVisitor::new(source_units.as_slice())),
            Box::new(analysis::ComparisonUtilizationVisitor),
            Box::new(analysis::AssignmentComparisonsVisitor),
            Box::new(analysis::StateVariableMutabilityVisitor::new(source_units.as_slice(), &call_graph)),
            Box::new(analysis::UnusedStateVariablesVisitor::default()),
        ],
        ..Default::default()
    };

    walker.analyze(source_units.as_slice())
}
