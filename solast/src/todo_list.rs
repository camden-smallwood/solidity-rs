use solidity::ast::*;

pub fn print(source_units: &[SourceUnit]) {
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