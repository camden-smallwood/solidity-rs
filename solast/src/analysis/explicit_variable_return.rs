use solidity::ast::*;
use std::{collections::HashSet, io};

pub struct ExplicitVariableReturnVisitor{
    variable_declarations: HashSet<NodeID>,
    reported_functions: HashSet<NodeID>,
}

impl Default for ExplicitVariableReturnVisitor {
    fn default() -> Self {
        Self {
            variable_declarations: HashSet::new(),
            reported_functions: HashSet::new(),
        }
    }
}

impl AstVisitor for ExplicitVariableReturnVisitor {
    fn visit_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> io::Result<()> {
        let definition_id = match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                function_definition.id
            }
            ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                modifier_definition.id
            }
            _ => return Ok(()),
        };

        match context.statement {
            Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                for declaration in variable_declaration_statement.declarations.iter() {
                    if let Some(declaration) = declaration {
                        if !self.variable_declarations.contains(&declaration.id) {
                            self.variable_declarations.insert(declaration.id);
                        }
                    }
                }
            }

            Statement::Return(return_statement) => {
                match return_statement.expression.as_ref() {
                    Some(Expression::Identifier(identifier)) => {
                        if self
                            .variable_declarations
                            .contains(&identifier.referenced_declaration)
                        {
                            if !self.reported_functions.contains(&definition_id) {
                                self.reported_functions.insert(definition_id);

                                match context.definition_node {
                                    ContractDefinitionNode::FunctionDefinition(function_definition) => {
                                        println!(
                                            "\tL{}: The {} `{}` {} returns the local `{}` variable explicitly",
                    
                                            context.current_source_unit.source_line(return_statement.src.as_str()).unwrap(),

                                            function_definition.visibility,

                                            if function_definition.name.is_empty() {
                                                format!("{}", context.contract_definition.name)
                                            } else {
                                                format!(
                                                    "{}.{}",
                                                    context.contract_definition.name, function_definition.name
                                                )
                                            },

                                            function_definition.kind,
                                            
                                            identifier.name
                                        );
                                    }

                                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                                        println!(
                                            "\tL{}: The {} `{}` modifier returns the local `{}` variable explicitly",

                                            context.current_source_unit.source_line(return_statement.src.as_str()).unwrap(),

                                            format!("{:?}", modifier_definition.visibility).to_lowercase(),

                                            if modifier_definition.name.is_empty() {
                                                format!("{}", context.contract_definition.name)
                                            } else {
                                                format!(
                                                    "{}.{}",
                                                    context.contract_definition.name, modifier_definition.name
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

                    Some(Expression::TupleExpression(tuple_expression)) => {
                        let mut all_local_variables = true;
                        let mut local_variable_names = vec![];

                        for component in tuple_expression.components.iter() {
                            if let Some(component) = component {
                                if let Expression::Identifier(identifier) = component {
                                    if self
                                        .variable_declarations
                                        .contains(&identifier.referenced_declaration)
                                    {
                                        local_variable_names
                                            .push(format!("`{}`", identifier.name.clone()));
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

                                match context.definition_node {
                                    ContractDefinitionNode::FunctionDefinition(function_definition) => {
                                        println!(
                                            "\tL{}: The {} `{}` {} returns the local {} variables explicitly",
                                            
                                            context.current_source_unit.source_line(return_statement.src.as_str()).unwrap(),

                                            function_definition.visibility,

                                            if function_definition.name.is_empty() {
                                                format!("{}", context.contract_definition.name)
                                            } else {
                                                format!(
                                                    "{}.{}",
                                                    context.contract_definition.name, function_definition.name
                                                )
                                            },

                                            function_definition.kind,

                                            local_variable_names.join(", ")
                                        );
                                    }

                                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                                        println!(
                                            "\tL{}: The {} `{}` modifier returns the local {} variables explicitly",

                                            context.current_source_unit.source_line(return_statement.src.as_str()).unwrap(),

                                            format!("{:?}", modifier_definition.visibility),

                                            if modifier_definition.name.is_empty() {
                                                format!("{}", context.contract_definition.name)
                                            } else {
                                                format!(
                                                    "{}.{}",
                                                    context.contract_definition.name, modifier_definition.name
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
