use std::{collections::HashSet, env, fs::File, io, path::PathBuf};

pub mod analysis;
pub mod truffle;

const VISITORS: &'static [(&'static str, for<'a> fn(&'a [solidity::ast::SourceUnit]) -> Box<dyn analysis::AstVisitor + 'a>)] = &[
    ("no_spdx_identifier", |_| Box::new(analysis::NoSpdxIdentifierVisitor)),
    ("floating_solidity_version", |_| Box::new(analysis::FloatingSolidityVersionVisitor)),
    ("node_modules_imports", |_| Box::new(analysis::NodeModulesImportsVisitor)),
    ("abstract_contracts", |_| Box::new(analysis::AbstractContractsVisitor)),
    ("large_literals", |_| Box::new(analysis::LargeLiteralsVisitor::default())),
    ("tight_variable_packing", |_| Box::new(analysis::TightVariablePackingVisitor::default())),
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
    ("manipulatable_balance_usage", |_| Box::new(analysis::ManipulatableBalanceUsageVisitor)),
];

fn main() -> io::Result<()> {
    let mut args = env::args();
    let _ = args.next().unwrap();

    let mut path: Option<PathBuf> = None;
    let mut todo_list = false;
    let mut visitor_names: HashSet<String> = HashSet::new();
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

                s if VISITORS.iter().find(|visitor| visitor.0 == s).is_some() => {
                    if !visitor_names.contains(s) {
                        visitor_names.insert(s.into());
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

    if truffle_config_path.is_file() {
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

                let mut lines = vec![];

                //
                // Print enums
                //

                for definition_node in contract_definition.nodes.iter() {
                    if let solidity::ast::ContractDefinitionNode::EnumDefinition(enum_definition) = definition_node {
                        lines.push(format!("- [ ] `enum {}`", enum_definition.name).to_string());
                    }
                }

                if !lines.is_empty() {
                    println!();
                    println!("#### Enums:");
                    
                    for line in lines.iter() {
                        println!("{}", line);
                    }

                    lines.clear();
                }

                //
                // Print structs
                //
                
                for definition_node in contract_definition.nodes.iter() {
                    if let solidity::ast::ContractDefinitionNode::StructDefinition(struct_definition) = definition_node {
                        lines.push(format!("- [ ] `struct {}`", struct_definition.name).to_string());
                    }
                }

                if !lines.is_empty() {
                    println!();
                    println!("#### Structs:");
                    
                    for line in lines.iter() {
                        println!("{}", line);
                    }

                    lines.clear();
                }
                
                //
                // Print variables
                //

                for definition_node in contract_definition.nodes.iter() {
                    if let solidity::ast::ContractDefinitionNode::VariableDeclaration(variable_declaration) = definition_node {
                        lines.push(format!("- [ ] `{}`", variable_declaration).to_string());
                    }
                }

                if !lines.is_empty() {
                    println!();
                    println!("#### Variables:");
                    
                    for line in lines.iter() {
                        println!("{}", line);
                    }

                    lines.clear();
                }

                //
                // Print modifiers
                //

                for definition_node in contract_definition.nodes.iter() {
                    if let solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) = definition_node {
                        let mut line = String::new();
                        
                        line.push_str("- [ ] `modifier");

                        if !modifier_definition.name.is_empty() {
                            line.push_str(format!(" {}", modifier_definition.name).as_str());
                        }

                        line.push_str(format!("{}", modifier_definition.parameters).as_str());
                
                        if modifier_definition.visibility != solidity::ast::Visibility::Internal {
                            line.push_str(format!("{} {}", modifier_definition.parameters, modifier_definition.visibility).as_str());
                        }
                        
                        if let Some(true) = modifier_definition.r#virtual {
                            line.push_str(format!(" virtual").as_str());
                        }
                
                        if let Some(overrides) = modifier_definition.overrides.as_ref() {
                            line.push_str(format!(" {}", overrides).as_str());
                        }
                        
                        line.push_str(format!("`").as_str());

                        lines.push(line);
                    }
                }
                
                if !lines.is_empty() {
                    println!();
                    println!("#### Modifiers:");
                    
                    for line in lines.iter() {
                        println!("{}", line);
                    }

                    lines.clear();
                }

                //
                // Print functions
                //

                for definition_node in contract_definition.nodes.iter() {
                    if let solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) = definition_node {
                        if function_definition.body.is_none() {
                            continue;
                        }

                        let mut line = String::new();

                        line.push_str(format!("- [ ] `{}", function_definition.kind).as_str());

                        if !function_definition.name.is_empty() {
                            line.push_str(format!(" {}", function_definition.name).as_str());
                        }
                
                        line.push_str(format!("{} {}", function_definition.parameters, function_definition.visibility).as_str());
                        
                        if function_definition.state_mutability != solidity::ast::StateMutability::NonPayable {
                            line.push_str(format!(" {}", function_definition.state_mutability).as_str());
                        }
                
                        if let Some(true) = function_definition.r#virtual {
                            line.push_str(format!(" virtual").as_str());
                        }
                
                        if let Some(overrides) = function_definition.overrides.as_ref() {
                            line.push_str(format!(" {}", overrides).as_str());
                        }
                
                        for modifier in function_definition.modifiers.iter() {
                            line.push_str(format!(" {}", modifier).as_str());
                        }
                
                        if !function_definition.return_parameters.parameters.is_empty() {
                            line.push_str(format!(" returns {}", function_definition.return_parameters).as_str());
                        }

                        line.push_str(format!("`").as_str());

                        lines.push(line);
                    }
                }

                if !lines.is_empty() {
                    println!();
                    println!("#### Functions:");
                    
                    for line in lines.iter() {
                        println!("{}", line);
                    }

                    lines.clear();
                }

                println!();
            }
        
            println!("---");
            println!();
        }
    }

    let mut visitors: Vec<Box<dyn analysis::AstVisitor>> = vec![
        Box::new(analysis::SourceUnitVisitor::new(source_units.as_slice())),
    ];

    for &(visitor_name, visitor_constructor_fn) in VISITORS {
        if visitor_names.is_empty() || visitor_names.contains(visitor_name) {
            visitors.push(visitor_constructor_fn(source_units.as_slice()));
        }
    }

    analysis::visit_source_units(visitors, source_units.as_slice())
}
