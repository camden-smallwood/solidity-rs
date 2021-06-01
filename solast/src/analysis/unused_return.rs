use super::AstVisitor;
use solidity::ast::SourceUnit;
use std::io;

pub struct UnusedReturnVisitor<'a> {
    source_units: &'a [SourceUnit],
}

impl<'a> UnusedReturnVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self { source_units }
    }
}

impl AstVisitor for UnusedReturnVisitor<'_> {
    fn visit_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        statement: &'a solidity::ast::Statement,
    ) -> io::Result<()> {
        if let solidity::ast::Statement::ExpressionStatement(expression_statement) = statement {
            if let solidity::ast::Expression::FunctionCall(solidity::ast::FunctionCall {
                expression,
                ..
            })
            | solidity::ast::Expression::FunctionCallOptions(solidity::ast::FunctionCallOptions {
                expression,
                ..
            }) = &expression_statement.expression {
                let referenced_declaration = match expression.root_expression() {
                    Some(solidity::ast::Expression::Identifier(solidity::ast::Identifier { referenced_declaration, .. })) => referenced_declaration.clone(),
                    Some(solidity::ast::Expression::MemberAccess(solidity::ast::MemberAccess { referenced_declaration: Some(referenced_delcaration), .. })) => referenced_delcaration.clone(),
                    _ => return Ok(())
                };

                for source_unit in self.source_units.iter() {
                    if let Some((called_contract_definition, called_function_definition)) = source_unit.function_and_contract_definition(referenced_declaration) {
                        if !called_function_definition.return_parameters.parameters.is_empty() {
                            match definition_node {
                                solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => {
                                    println!(
                                        "\t{} {} {} makes a call to the {} {} {}, ignoring the returned {}",

                                        format!("{:?}", function_definition.visibility),

                                        if function_definition.name.is_empty() {
                                            format!("{}", contract_definition.name)
                                        } else {
                                            format!("{}.{}", contract_definition.name, function_definition.name)
                                        },

                                        format!("{:?}", function_definition.kind).to_lowercase(),

                                        format!("{:?}", called_function_definition.visibility).to_lowercase(),

                                        if called_function_definition.name.is_empty() {
                                            format!("{}", called_contract_definition.name)
                                        } else {
                                            format!("{}.{}", called_contract_definition.name, called_function_definition.name)
                                        },

                                        format!("{:?}", called_function_definition.kind).to_lowercase(),

                                        if called_function_definition.return_parameters.parameters.len() > 1 {
                                            "values"
                                        } else {
                                            "value"
                                        }
                                    );
                                }

                                solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                                    println!(
                                        "\t{} {} modifier makes a call to the {} {} {}, ignoring the returned {}",

                                        format!("{:?}", modifier_definition.visibility),

                                        if modifier_definition.name.is_empty() {
                                            format!("{}", contract_definition.name)
                                        } else {
                                            format!("{}.{}", contract_definition.name, modifier_definition.name)
                                        },

                                        format!("{:?}", called_function_definition.visibility).to_lowercase(),

                                        if called_function_definition.name.is_empty() {
                                            format!("{}", called_contract_definition.name)
                                        } else {
                                            format!("{}.{}", called_contract_definition.name, called_function_definition.name)
                                        },

                                        format!("{:?}", called_function_definition.kind).to_lowercase(),

                                        if called_function_definition.return_parameters.parameters.len() > 1 {
                                            "values"
                                        } else {
                                            "value"
                                        }
                                    );
                                }

                                _ => {}
                            }

                            return Ok(());
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
