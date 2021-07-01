use super::AstVisitor;
use solidity::ast::*;

pub struct ManipulatableBalanceUsageVisitor;

//
// TODO:
//   determine if token.balanceOf(address(this)) is used in ways which can be manipulated
//

impl AstVisitor for ManipulatableBalanceUsageVisitor {
    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        function_call: &'a FunctionCall,
    ) -> std::io::Result<()> {
        if function_call.arguments.len() != 1 {
            return Ok(())
        }

        match function_call.expression.as_ref() {
            Expression::MemberAccess(MemberAccess {
                member_name,
                ..
            }) if member_name == "balanceOf" => match function_call.arguments.first().unwrap() {
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
                        }) if name == "this" => match definition_node {
                            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                                "\tThe {} {} in the `{}` {} contains usage of `.balanceOf(address(this))`",

                                function_definition.visibility,

                                if let FunctionKind::Constructor = function_definition.kind {
                                    format!("{}", "constructor")
                                } else {
                                    format!("`{}` {}", function_definition.name, function_definition.kind)
                                },

                                contract_definition.name,
                                contract_definition.kind
                            ),

                            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                                "\tThe `{}` modifier in the `{}` {} contains usage of `.balanceOf(address(this))`",
                                modifier_definition.name,
                                contract_definition.name,
                                contract_definition.kind
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
