use super::AstVisitor;
use crate::truffle;
use solidity::ast::NodeID;
use std::{collections::HashSet, io};

pub struct ExplicitVariableReturnVisitor<'a> {
    pub files: &'a [truffle::File],
    variable_declarations: HashSet<NodeID>,
    reported_functions: HashSet<NodeID>,
}

impl<'a> ExplicitVariableReturnVisitor<'a> {
    pub fn new(files: &'a [truffle::File]) -> Self {
        Self {
            files,
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
        function_definition: &'a solidity::ast::FunctionDefinition,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        statement: &'a solidity::ast::Statement,
    ) -> io::Result<()> {
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
                            if !self.reported_functions.contains(&function_definition.id) {
                                self.reported_functions.insert(function_definition.id);

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
                            if !self.reported_functions.contains(&function_definition.id) {
                                self.reported_functions.insert(function_definition.id);

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
