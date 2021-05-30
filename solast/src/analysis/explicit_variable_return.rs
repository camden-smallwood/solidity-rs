use super::AstVisitor;
use solidity::ast::{NodeID, SourceUnit};
use std::{collections::HashSet, io};

pub struct ExplicitVariableReturnVisitor<'a> {
    pub source_units: &'a [SourceUnit],
    variable_declarations: HashSet<NodeID>,
    reported_functions: HashSet<NodeID>,
}

impl<'a> ExplicitVariableReturnVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self {
            source_units,
            variable_declarations: HashSet::new(),
            reported_functions: HashSet::new(),
        }
    }
}

impl AstVisitor for ExplicitVariableReturnVisitor<'_> {
    fn visit_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        statement: &'a solidity::ast::Statement,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => {
                function_definition.id
            }
            solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                modifier_definition.id
            }
            _ => return Ok(()),
        };

        match statement {
            solidity::ast::Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                for declaration in variable_declaration_statement.declarations.iter() {
                    if let Some(declaration) = declaration {
                        if !self.variable_declarations.contains(&declaration.id) {
                            self.variable_declarations.insert(declaration.id);
                        }
                    }
                }
            }

            solidity::ast::Statement::Return(return_statement) => {
                match return_statement.expression.as_ref() {
                    Some(solidity::ast::Expression::Identifier(identifier)) => {
                        if self
                            .variable_declarations
                            .contains(&identifier.referenced_declaration)
                        {
                            if !self.reported_functions.contains(&definition_id) {
                                self.reported_functions.insert(definition_id);

                                match definition_node {
                                    solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => {
                                        println!(
                                            "\t{} {} {} returns local '{}' variable explicitly",
                                            format!("{:?}", function_definition.visibility),
                                            if function_definition.name.is_empty() {
                                                format!("{}", contract_definition.name)
                                            } else {
                                                format!(
                                                    "{}.{}",
                                                    contract_definition.name, function_definition.name
                                                )
                                            },
                                            format!("{:?}", function_definition.kind).to_lowercase(),
                                            identifier.name
                                        );
                                    }

                                    solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                                        println!(
                                            "\t{} {} modifier returns local '{}' variable explicitly",
                                            format!("{:?}", modifier_definition.visibility),
                                            if modifier_definition.name.is_empty() {
                                                format!("{}", contract_definition.name)
                                            } else {
                                                format!(
                                                    "{}.{}",
                                                    contract_definition.name, modifier_definition.name
                                                )
                                            },
                                            identifier.name
                                        );
                                    }

                                    _ => ()
                                }
                            }
                        }
                    }

                    Some(solidity::ast::Expression::TupleExpression(tuple_expression)) => {
                        let mut all_local_variables = true;
                        let mut local_variable_names = vec![];

                        for component in tuple_expression.components.iter() {
                            if let Some(component) = component {
                                if let solidity::ast::Expression::Identifier(identifier) = component {
                                    if self
                                        .variable_declarations
                                        .contains(&identifier.referenced_declaration)
                                    {
                                        local_variable_names
                                            .push(format!("'{}'", identifier.name.clone()));
                                    } else {
                                        all_local_variables = false;
                                        break;
                                    }
                                } else {
                                    all_local_variables = false;
                                    break;
                                }
                            }
                        }

                        if all_local_variables {
                            if !self.reported_functions.contains(&definition_id) {
                                self.reported_functions.insert(definition_id);

                                match definition_node {
                                    solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => {
                                        println!(
                                            "\t{} {} {} returns local {} variables explicitly",
                                            format!("{:?}", function_definition.visibility),
                                            if function_definition.name.is_empty() {
                                                format!("{}", contract_definition.name)
                                            } else {
                                                format!(
                                                    "{}.{}",
                                                    contract_definition.name, function_definition.name
                                                )
                                            },
                                            format!("{:?}", function_definition.kind).to_lowercase(),
                                            local_variable_names.join(", ")
                                        );
                                    }

                                    solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                                        println!(
                                            "\t{} {} modifier returns local {} variables explicitly",
                                            format!("{:?}", modifier_definition.visibility),
                                            if modifier_definition.name.is_empty() {
                                                format!("{}", contract_definition.name)
                                            } else {
                                                format!(
                                                    "{}.{}",
                                                    contract_definition.name, modifier_definition.name
                                                )
                                            },
                                            local_variable_names.join(", ")
                                        );
                                    }

                                    _ => ()
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }

        Ok(())
    }
}
