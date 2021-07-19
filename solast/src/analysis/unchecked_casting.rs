use solidity::ast::*;
use std::io;

pub struct UncheckedCastingVisitor;

impl AstVisitor for UncheckedCastingVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if context.function_call.kind != FunctionCallKind::TypeConversion {
            return Ok(())
        }

        let (type_descriptions, type_name) = match context.function_call.expression.as_ref() {
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_name: TypeName::ElementaryTypeName(type_name),
                type_descriptions,
                ..
            }) => (type_descriptions, type_name),

            _ => return Ok(())
        };

        if type_name.name.starts_with("int") || type_name.name.starts_with("uint") {
            //
            // TODO: if the argument is a non-literal, verify its expression was
            //       checked for validity via require, if/else or a conditional
            //
        }

        //
        // Check for redundant cast (i.e: casting uint256 to uint256)
        //

        if let Some(argument_expression) = context.function_call.arguments.first() {
            if let Some(argument_type_descriptions) = argument_expression.type_descriptions() {
                if type_descriptions == argument_type_descriptions {
                    match context.definition_node {
                        ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                            "\tL{}: The {} {} in the `{}` {} contains a redundant cast: `{}`",

                            context.current_source_unit.source_line(context.function_call.src.as_str()).unwrap(),

                            function_definition.visibility,

                            if let FunctionKind::Constructor = function_definition.kind {
                                format!("{}", "constructor")
                            } else {
                                format!("`{}` {}", function_definition.name, function_definition.kind)
                            },

                            context.contract_definition.name,
                            context.contract_definition.kind,

                            context.function_call
                        ),

                        ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                            "\tL{}: The `{}` modifier in the `{}` {} contains a redundant cast: `{}`",

                            context.current_source_unit.source_line(context.function_call.src.as_str()).unwrap(),

                            modifier_definition.name,

                            context.contract_definition.name,
                            context.contract_definition.kind,

                            context.function_call
                        ),

                        _ => ()
                    }
                }
            }
        }

        Ok(())
    }
}
