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
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} contains `{}` usage, which can be optimized with assembly: `{}`",

                source_line,

                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    "constructor".to_string()
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },

                contract_definition.name,
                contract_definition.kind,

                expression,

                if external {
                    "assembly { bal := balance(addr); }"
                } else {
                    "assembly { bal := selfbalance(); }"
                }
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The `{}` modifier in the `{}` {} contains `{}` usage, which can be optimized with assembly: `{}`",

                source_line,

                modifier_definition.name,

                contract_definition.name,
                contract_definition.kind,

                expression,

                if external {
                    "assembly { bal := balance(addr); }"
                } else {
                    "assembly { bal := selfbalance(); }"
                }
            ),

            _ => {}
        }
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
                expression,
                name != "this"
            );
        }

        Ok(())
    }
}
