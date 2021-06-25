use std::{collections::HashSet, env, fs::File, io, path::PathBuf};

pub mod analysis;
pub mod truffle;

const ANALYZERS: &'static [(&'static str, for<'a> fn(&'a [solidity::ast::SourceUnit]) -> Box<dyn analysis::AstVisitor + 'a>)] = &[
    ("no_spdx_identifier", |_| Box::new(analysis::NoSpdxIdentifierVisitor)),
    ("floating_solidity_version", |_| Box::new(analysis::FloatingSolidityVersionVisitor)),
    ("node_modules_imports", |_| Box::new(analysis::NodeModulesImportsVisitor)),
    ("abstract_contracts", |_| Box::new(analysis::AbstractContractsVisitor)),
    ("large_literals", |_| Box::new(analysis::LargeLiteralsVisitor::default())),
    ("redundant_getter_function", |source_units| Box::new(analysis::RedundantGetterFunctionVisitor::new(source_units))),
    ("require_without_message", |source_units| Box::new(analysis::RequireWithoutMessageVisitor::new(source_units))),
    ("state_variable_shadowing", |source_units| Box::new(analysis::StateVariableShadowingVisitor::new(source_units))),
    ("explicit_variable_return", |_| Box::new(analysis::ExplicitVariableReturnVisitor::default())),
    ("unused_return", |source_units| Box::new(analysis::UnusedReturnVisitor::new(source_units))),
    ("storage_array_loop", |source_units| Box::new(analysis::StorageArrayLoopVisitor::new(source_units))),
    ("external_calls_in_loop", |source_units| Box::new(analysis::ExternalCallsInLoopVisitor::new(source_units))),
    ("check_effects_interactions", |source_units| Box::new(analysis::CheckEffectsInteractionsVisitor::new(source_units))),
    ("raw_address_transfer", |source_units| Box::new(analysis::RawAddressTransferVisitor::new(source_units))),
    ("safe_erc20_functions", |source_units| Box::new(analysis::SafeERC20FunctionsVisitor::new(source_units))),
    ("unchecked_erc20_transfer", |source_units| Box::new(analysis::UncheckedERC20TransferVisitor::new(source_units))),
    ("unpaid_payable_functions", |source_units| Box::new(analysis::UnpaidPayableFunctionsVisitor::new(source_units))),
    ("divide_before_multiply", |_| Box::new(analysis::DivideBeforeMultiplyVisitor)),
    ("comparison_utilization", |_| Box::new(analysis::ComparisonUtilizationVisitor)),
    ("assignment_comparisons", |_| Box::new(analysis::AssignmentComparisonsVisitor)),
    ("state_variable_mutability", |source_units| Box::new(analysis::StateVariableMutabilityVisitor::new(source_units))),
    ("unused_state_variables", |_| Box::new(analysis::UnusedStateVariablesVisitor::default())),
    ("ineffectual_statements", |_| Box::new(analysis::IneffectualStatementsVisitor)),
    ("inline_assembly", |_| Box::new(analysis::InlineAssemblyVisitor::default())),
    ("unchecked_casting", |_| Box::new(analysis::UncheckedCastingVisitor)),
    ("unnecessary_pragmas", |_| Box::new(analysis::UnnecessaryPragmasVisitor)),
    ("missing_return", |_| Box::new(analysis::MissingReturnVisitor::default())),
    ("redundant_state_variable_access", |_| Box::new(analysis::RedundantStateVariableAccessVisitor)),
    ("unnecessary_comparisons", |_| Box::new(analysis::UnnecessaryComparisonsVisitor)),
    ("assert_usage", |_| Box::new(analysis::AssertUsageVisitor::default())),
    ("unrestricted_setter_functions", |_| Box::new(analysis::UnrestrictedSetterFunctionsVisitor)),
];

fn main() -> io::Result<()> {
    let mut args = env::args();
    let _ = args.next().unwrap();

    let mut path: Option<PathBuf> = None;
    let mut todo_list = false;
    let mut analyzer_names: HashSet<String> = HashSet::new();
    let mut contract_name: Option<String> = None;

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

                s if s.starts_with("contract=") => {
                    if contract_name.is_some() {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Multiple contracts specified: {} {}", path.unwrap().to_string_lossy(), arg)));
                    }
                    
                    contract_name = Some(s.trim_start_matches("contract=").into());
                }

                s if ANALYZERS.iter().find(|analyzer| analyzer.0 == s).is_some() => {
                    if !analyzer_names.contains(s) {
                        analyzer_names.insert(s.into());
                    }
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
                if let Some(contract_name) = contract_name.as_ref().map(String::as_str) {
                    if !source_unit.contract_definitions().iter().find(|c| c.name == contract_name).is_some() {
                        continue;
                    }
                }
                
                source_units.push(source_unit);
            }
        }

        source_units.sort_by(|lhs, rhs| lhs.absolute_path.as_ref().map(String::as_str).unwrap_or("").cmp(rhs.absolute_path.as_ref().map(String::as_str).unwrap_or("")));
    } else {
        todo!("truffle config not found; implement support for other project types")
    }

    if todo_list {
        for source_unit in source_units.iter() {
            for contract_definition in source_unit.contract_definitions() {
                if let solidity::ast::ContractKind::Interface = contract_definition.kind {
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

    let mut walker = analysis::AstWalker {
        visitors: vec![
            Box::new(analysis::SourceUnitVisitor::new(source_units.as_slice())),
        ],
        ..Default::default()
    };

    for &(analyzer_name, analyzer_constructor_fn) in ANALYZERS {
        if analyzer_names.is_empty() || analyzer_names.contains(analyzer_name) {
            walker.visitors.push(analyzer_constructor_fn(source_units.as_slice()));
        }
    }

    walker.analyze(source_units.as_slice())
}
