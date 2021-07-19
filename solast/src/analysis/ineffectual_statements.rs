use solidity::ast::*;
use std::io;

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
                        "\tL{}: {} {} {} contains an ineffectual literal statement: {}",

                        context.current_source_unit.source_line(literal.src.as_str()).unwrap(),

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
                        "\tL{}: {} {} {} contains an ineffectual identifier statement: {}",

                        context.current_source_unit.source_line(identifier.src.as_str()).unwrap(),

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
                        "\tL{}: {} {} {} contains an ineffectual index access statement: {}",

                        context.current_source_unit.source_line(index_access.src.as_str()).unwrap(),

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
                        "\tL{}: {} {} {} contains an ineffectual member access statement: {}",

                        context.current_source_unit.source_line(member_access.src.as_str()).unwrap(),

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
                        "\tL{}: {} {} {} contains an ineffectual binary operation statement: {}",

                        context.current_source_unit.source_line(binary_operation.src.as_str()).unwrap(),

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
                        "\tL{}: {} {} {} contains an ineffectual conditional statement: {}",

                        context.current_source_unit.source_line(conditional.src.as_str()).unwrap(),

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
                        "\tL{}: {} {} {} contains an ineffectual tuple expression statement: {}",

                        context.current_source_unit.source_line(tuple_expression.src.as_str()).unwrap(),

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
                
                solidity::ast::Expression::FunctionCallOptions(function_call_options) => {
                    if function_call_options.arguments.is_none() {
                        println!(
                            "\tL{}: {} {} {} contains an ineffectual function call expression statement: {}",

                            context.current_source_unit.source_line(function_call_options.src.as_str()).unwrap(),

                            format!("{:?}", function_definition.visibility),
                            
                            if function_definition.name.is_empty() {
                                format!("{}", context.contract_definition.name)
                            } else {
                                format!("{}.{}", context.contract_definition.name, function_definition.name)
                            },

                            function_definition.kind,

                            function_call_options
                        );
                    }
                }

                _ => {}
            }
        }

        Ok(())
    }
}
