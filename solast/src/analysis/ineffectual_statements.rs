use solidity::ast::{ContractDefinitionNode, ExpressionStatement, Statement};
use std::io;
use super::{AstVisitor, StatementContext};

pub struct IneffectualStatementsVisitor;

impl AstVisitor for IneffectualStatementsVisitor {
    fn visit_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> io::Result<()> {
        let function_definition = match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(())
        };

        if let Statement::ExpressionStatement(ExpressionStatement { expression }) = context.statement {
            match expression {
                solidity::ast::Expression::Literal(literal) => {
                    println!(
                        "\t{} {} {} contains an ineffectual literal statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
                        },
                        function_definition.kind,
                        literal
                    );
                }
                
                solidity::ast::Expression::Identifier(identifier) => {
                    println!(
                        "\t{} {} {} contains an ineffectual identifier statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
                        },
                        function_definition.kind,
                        identifier
                    );
                }

                solidity::ast::Expression::IndexAccess(index_access) => {
                    println!(
                        "\t{} {} {} contains an ineffectual index access statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
                        },
                        function_definition.kind,
                        index_access
                    );
                }

                solidity::ast::Expression::IndexRangeAccess(index_range_access) => {
                    println!(
                        "\t{} {} {} contains an ineffectual index range access statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
                        },
                        function_definition.kind,
                        index_range_access
                    );
                }

                solidity::ast::Expression::MemberAccess(member_access) => {
                    println!(
                        "\t{} {} {} contains an ineffectual member access statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
                        },
                        function_definition.kind,
                        member_access
                    );
                }

                solidity::ast::Expression::BinaryOperation(binary_operation) => {
                    println!(
                        "\t{} {} {} contains an ineffectual binary operation statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
                        },
                        function_definition.kind,
                        binary_operation
                    );
                }

                solidity::ast::Expression::Conditional(conditional) => {
                    println!(
                        "\t{} {} {} contains an ineffectual conditional statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
                        },
                        function_definition.kind,
                        conditional
                    );
                }

                solidity::ast::Expression::TupleExpression(tuple_expression) => {
                    println!(
                        "\t{} {} {} contains an ineffectual tuple expression statement: {}",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.name.is_empty() {
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
                        },
                        function_definition.kind,
                        tuple_expression
                    );
                }
                
                _ => {}
            }
        }

        Ok(())
    }
}
