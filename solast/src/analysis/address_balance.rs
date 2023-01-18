use solidity::ast::*;

pub struct AddressBalanceVisitor;

impl AddressBalanceVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        expression: &dyn std::fmt::Display,
        external: bool,
    ) {
        println!(
            "\t{} contains `{}` usage, which can be optimized with assembly: `{}`",
            contract_definition.definition_node_location(source_line, definition_node),
            expression,
            if external {
                "assembly { bal := balance(addr); }"
            } else {
                "assembly { bal := selfbalance(); }"
            }
        );
    }
}

impl AstVisitor for AddressBalanceVisitor {
    fn visit_member_access<'a, 'b>(&mut self, context: &mut MemberAccessContext<'a, 'b>) -> std::io::Result<()> {
        if context.member_access.member_name != "balance" {
            return Ok(())
        }

        let (expression, arguments) = match context.member_access.expression.as_ref() {
            Expression::FunctionCall(FunctionCall {
                expression,
                arguments,
                ..
            }) if arguments.len() == 1 => (expression, arguments),

            _ => return Ok(())
        };
    
        match expression.as_ref() {
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_name: TypeName::ElementaryTypeName(ElementaryTypeName {
                    name,
                    ..
                }),
                ..
            }) if name == "address" => {}

            _ => return Ok(())
        }
        
        if let Some(Expression::Identifier(Identifier {
            name,
            ..
        })) = arguments.first() {
            self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.member_access.src.as_str())?,
                context.member_access,
                name != "this"
            );
        }

        Ok(())
    }
}
