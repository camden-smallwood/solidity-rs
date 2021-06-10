use solidity::ast::{Block, ContractDefinition, ContractDefinitionNode, ExpressionStatement, SourceUnit, Statement};
use std::io;
use super::AstVisitor;

pub struct IneffectualStatementsVisitor;

impl AstVisitor for IneffectualStatementsVisitor {
    fn visit_statement<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
    ) -> io::Result<()> {
        let function_definition = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(())
        };

        if let Statement::ExpressionStatement(ExpressionStatement { expression }) = statement {
            match expression {
                solidity::ast::Expression::Literal(literal) => {
                    println!(
                        "\t{} {} {} contains an ineffectual literal statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", contract_definition.name)
                        } else {
                            format!("{}.{}", contract_definition.name, function_definition.name)
                        },
                        format!("{:?}", function_definition.kind).to_lowercase(),
                        literal
                    );
                }
                
                solidity::ast::Expression::Identifier(identifier) => {
                    println!(
                        "\t{} {} {} contains an ineffectual identifier statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", contract_definition.name)
                        } else {
                            format!("{}.{}", contract_definition.name, function_definition.name)
                        },
                        format!("{:?}", function_definition.kind).to_lowercase(),
                        identifier
                    );
                }

                solidity::ast::Expression::IndexAccess(index_access) => {
                    println!(
                        "\t{} {} {} contains an ineffectual index access statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", contract_definition.name)
                        } else {
                            format!("{}.{}", contract_definition.name, function_definition.name)
                        },
                        format!("{:?}", function_definition.kind).to_lowercase(),
                        index_access
                    );
                }

                solidity::ast::Expression::IndexRangeAccess(index_range_access) => {
                    println!(
                        "\t{} {} {} contains an ineffectual index range access statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", contract_definition.name)
                        } else {
                            format!("{}.{}", contract_definition.name, function_definition.name)
                        },
                        format!("{:?}", function_definition.kind).to_lowercase(),
                        index_range_access
                    );
                }

                solidity::ast::Expression::MemberAccess(member_access) => {
                    println!(
                        "\t{} {} {} contains an ineffectual member access statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", contract_definition.name)
                        } else {
                            format!("{}.{}", contract_definition.name, function_definition.name)
                        },
                        format!("{:?}", function_definition.kind).to_lowercase(),
                        member_access
                    );
                }

                solidity::ast::Expression::BinaryOperation(binary_operation) => {
                    println!(
                        "\t{} {} {} contains an ineffectual binary operation statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", contract_definition.name)
                        } else {
                            format!("{}.{}", contract_definition.name, function_definition.name)
                        },
                        format!("{:?}", function_definition.kind).to_lowercase(),
                        binary_operation
                    );
                }

                solidity::ast::Expression::Conditional(conditional) => {
                    println!(
                        "\t{} {} {} contains an ineffectual conditional statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", contract_definition.name)
                        } else {
                            format!("{}.{}", contract_definition.name, function_definition.name)
                        },
                        format!("{:?}", function_definition.kind).to_lowercase(),
                        conditional
                    );
                }

                solidity::ast::Expression::TupleExpression(tuple_expression) => {
                    println!(
                        "\t{} {} {} contains an ineffectual tuple expression statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", contract_definition.name)
                        } else {
                            format!("{}.{}", contract_definition.name, function_definition.name)
                        },
                        format!("{:?}", function_definition.kind).to_lowercase(),
                        tuple_expression
                    );
                }
                
                _ => {}
            }
        }

        Ok(())
    }
}
