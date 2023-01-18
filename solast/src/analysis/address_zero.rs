use solidity::ast::*;

pub struct AddressZeroVisitor;

impl AddressZeroVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        expression: &dyn std::fmt::Display,
    ) {
        println!(
            "\t{} contains `{}` usage, which can be optimized with assembly: `{}`",
            contract_definition.definition_node_location(source_line, definition_node),
            expression,
            "assembly { if iszero(addr) { ... } }",
        );
    }
}

impl AstVisitor for AddressZeroVisitor {
    fn visit_binary_operation<'a, 'b>(
        &mut self,
        context: &mut BinaryOperationContext<'a, 'b>,
    ) -> std::io::Result<()> {
        if !matches!(context.binary_operation.operator.as_str(), "==" | "!=") {
            return Ok(());
        }

        let check_expression = |expression: &Expression| -> bool {
            match expression {
                Expression::FunctionCall(FunctionCall {
                    kind: FunctionCallKind::TypeConversion,
                    arguments,
                    expression,
                    ..
                }) if arguments.len() == 1 => match expression.as_ref() {
                    Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                        type_name:
                            TypeName::ElementaryTypeName(ElementaryTypeName {
                                name: type_name, ..
                            }),
                        ..
                    }) if type_name == "address" => match &arguments[0] {
                        Expression::Literal(Literal {
                            value: Some(value), ..
                        }) if value == "0" => true,

                        _ => false,
                    },

                    _ => false,
                },

                _ => false,
            }
        };

        if check_expression(context.binary_operation.left_expression.as_ref())
            || check_expression(context.binary_operation.right_expression.as_ref())
        {
            self.print_message(
                context.contract_definition,
                context.definition_node,
                context
                    .current_source_unit
                    .source_line(context.binary_operation.src.as_str())?,
                context.binary_operation,
            );
        }

        Ok(())
    }
}
