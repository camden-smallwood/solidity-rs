use super::{AstVisitor, FunctionCallContext, MemberAccessContext};
use solidity::ast::*;
use std::io;

pub struct ManipulatableBalanceUsageVisitor;

//
// TODO:
// * determine if balance can actually be manipulated
// * determine if manipulating balance has consequences
//

impl AstVisitor for ManipulatableBalanceUsageVisitor {
    fn visit_member_access<'a, 'b>(&mut self, context: &mut MemberAccessContext<'a, 'b>) -> io::Result<()> {
        if context.member_access.member_name == "balance" {
            if let Expression::FunctionCall(FunctionCall {
                expression,
                arguments,
                ..
            }) = context.member_access.expression.as_ref() {
                if let Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                    type_name: TypeName::ElementaryTypeName(ElementaryTypeName { name, .. }),
                    ..
                }) = expression.as_ref() {
                    if name == "address" && arguments.len() == 1 {
                        if let Expression::Identifier(Identifier {
                            name,
                            ..
                        }) = arguments.first().unwrap() {
                            if name == "this" {
                                match context.definition_node {
                                    ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                                        "\tL{}: The {} {} in the `{}` {} contains manipulatable balance usage: `{}`",

                                        context.current_source_unit.source_line(context.member_access.src.as_str()).unwrap(),

                                        function_definition.visibility,

                                        if let FunctionKind::Constructor = function_definition.kind {
                                            format!("{}", "constructor")
                                        } else {
                                            format!("`{}` {}", function_definition.name, function_definition.kind)
                                        },

                                        context.contract_definition.name,
                                        context.contract_definition.kind,

                                        context.member_access
                                    ),

                                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                                        "\tL{}: The `{}` modifier in the `{}` {} contains manipulatable balance usage: `{}`",

                                        context.current_source_unit.source_line(context.member_access.src.as_str()).unwrap(),

                                        modifier_definition.name,

                                        context.contract_definition.name,
                                        context.contract_definition.kind,

                                        context.member_access
                                    ),

                                    _ => ()
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if context.function_call.arguments.len() != 1 {
            return Ok(())
        }

        match context.function_call.expression.as_ref() {
            Expression::MemberAccess(MemberAccess {
                member_name,
                ..
            }) if member_name == "balanceOf" => match context.function_call.arguments.first().unwrap() {
                Expression::FunctionCall(FunctionCall {
                    expression,
                    arguments,
                    ..
                }) => match expression.as_ref() {
                    Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                        type_name: TypeName::ElementaryTypeName(ElementaryTypeName { name, .. }),
                        ..
                    }) if name == "address" && arguments.len() == 1 => match arguments.first().unwrap() {
                        Expression::Identifier(Identifier {
                            name,
                            ..
                        }) if name == "this" => match context.definition_node {
                            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                                "\tL{}: The {} {} in the `{}` {} contains manipulatable balance usage: `{}`",

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
                                "\tL{}: The `{}` modifier in the `{}` {} contains manipulatable balance usage: `{}`",

                                context.current_source_unit.source_line(context.function_call.src.as_str()).unwrap(),

                                modifier_definition.name,

                                context.contract_definition.name,
                                context.contract_definition.kind,

                                context.function_call
                            ),

                            _ => return Ok(())
                        }

                        _ => return Ok(())
                    }
    
                    _ => return Ok(())
                }
    
                _ => return Ok(())
            }

            _ => return Ok(())
        }

        Ok(())
    }
}
