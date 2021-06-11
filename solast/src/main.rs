use std::{collections::HashSet, env, fs::File, io, path::PathBuf};

pub mod analysis;
pub mod truffle;

fn main() -> io::Result<()> {
    let mut args = env::args();
    let _ = args.next().unwrap();

    let mut path: Option<PathBuf> = None;
    let mut todo_list = false;
    let mut analyzer_names: HashSet<String> = HashSet::new();

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

                s if s == "no_spdx_identifier"
                    || s == "floating_solidity_version"
                    || s == "node_modules_imports"
                    || s == "abstract_contracts"
                    || s == "large_literals"
                    || s == "redundant_getter_function"
                    || s == "require_without_message"
                    || s == "state_variable_shadowing"
                    || s == "explicit_variable_return"
                    || s == "unused_return"
                    || s == "storage_array_loop"
                    || s == "external_calls_in_loop"
                    || s == "check_effects_interactions"
                    || s == "raw_address_transfer"
                    || s == "safe_erc20_functions"
                    || s == "unchecked_erc20_transfer"
                    || s == "unpaid_payable_functions"
                    || s == "divide_before_multiply"
                    || s == "comparison_utilization"
                    || s == "assignment_comparisons"
                    || s == "state_variable_mutability"
                    || s == "unused_state_variables"
                    || s == "ineffectual_statements"
                    || s == "inline_assembly" =>
                {
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

    let mut walker = analysis::AstWalker {
        visitors: vec![
            Box::new(analysis::SourceUnitVisitor::new(source_units.as_slice())),
        ],
        ..Default::default()
    };

    if analyzer_names.is_empty() || analyzer_names.contains("no_spdx_identifier") {
        walker.visitors.push(Box::new(analysis::NoSpdxIdentifierVisitor));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("floating_solidity_version") {
        walker.visitors.push(Box::new(analysis::FloatingSolidityVersionVisitor));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("node_modules_imports") {
        walker.visitors.push(Box::new(analysis::NodeModulesImportsVisitor));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("abstract_contracts") {
        walker.visitors.push(Box::new(analysis::AbstractContractsVisitor));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("large_literals") {
        walker.visitors.push(Box::new(analysis::LargeLiteralsVisitor::default()));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("redundant_getter_function") {
        walker.visitors.push(Box::new(analysis::RedundantGetterFunctionVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("require_without_message") {
        walker.visitors.push(Box::new(analysis::RequireWithoutMessageVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("state_variable_shadowing") {
        walker.visitors.push(Box::new(analysis::StateVariableShadowingVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("explicit_variable_return") {
        walker.visitors.push(Box::new(analysis::ExplicitVariableReturnVisitor::default()));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("unused_return") {
        walker.visitors.push(Box::new(analysis::UnusedReturnVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("storage_array_loop") {
        walker.visitors.push(Box::new(analysis::StorageArrayLoopVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("external_calls_in_loop") {
        walker.visitors.push(Box::new(analysis::ExternalCallsInLoopVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("check_effects_interactions") {
        walker.visitors.push(Box::new(analysis::CheckEffectsInteractionsVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("raw_address_transfer") {
        walker.visitors.push(Box::new(analysis::RawAddressTransferVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("safe_erc20_functions") {
        walker.visitors.push(Box::new(analysis::SafeERC20FunctionsVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("unchecked_erc20_transfer") {
        walker.visitors.push(Box::new(analysis::UncheckedERC20TransferVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("unpaid_payable_functions") {
        walker.visitors.push(Box::new(analysis::UnpaidPayableFunctionsVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("divide_before_multiply") {
        walker.visitors.push(Box::new(analysis::DivideBeforeMultiplyVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("comparison_utilization") {
        walker.visitors.push(Box::new(analysis::ComparisonUtilizationVisitor));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("assignment_comparisons") {
        walker.visitors.push(Box::new(analysis::AssignmentComparisonsVisitor));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("state_variable_mutability") {
        walker.visitors.push(Box::new(analysis::StateVariableMutabilityVisitor::new(source_units.as_slice())));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("unused_state_variables") {
        walker.visitors.push(Box::new(analysis::UnusedStateVariablesVisitor::default()));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("ineffectual_statements") {
        walker.visitors.push(Box::new(analysis::IneffectualStatementsVisitor));
    }

    if analyzer_names.is_empty() || analyzer_names.contains("inline_assembly") {
        walker.visitors.push(Box::new(analysis::InlineAssemblyVisitor));
    }

    walker.analyze(source_units.as_slice())
}
