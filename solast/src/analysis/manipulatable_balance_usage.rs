use super::AstVisitor;

pub struct ManipulatableBalanceUsageVisitor;

//
// TODO:
//   determine if token.balanceOf(address(this)) is used in ways which can be manipulated
//

impl AstVisitor for ManipulatableBalanceUsageVisitor {
    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        function_call: &'a solidity::ast::FunctionCall,
    ) -> std::io::Result<()> {

        match function_call.expression.as_ref() {
            solidity::ast::Expression::MemberAccess(solidity::ast::MemberAccess {
                member_name,
                ..
            }) if member_name == "balanceOf" => {}

            _ => return Ok(()),
        }

        if function_call.arguments.len() != 1 {
            return Ok(());
        }

        match function_call.arguments.first().unwrap() {
            solidity::ast::Expression::FunctionCall(solidity::ast::FunctionCall {
                expression,
                arguments,
                ..
            }) => match expression.as_ref() {
                solidity::ast::Expression::ElementaryTypeNameExpression(
                    solidity::ast::ElementaryTypeNameExpression {
                        type_name:
                            solidity::ast::TypeName::ElementaryTypeName(
                                solidity::ast::ElementaryTypeName { name, .. },
                            ),
                        ..
                    },
                ) if name == "address" && arguments.len() == 1 => {
                    match arguments.first().unwrap() {
                        solidity::ast::Expression::Identifier(solidity::ast::Identifier {
                            name,
                            ..
                        }) if name == "this" => {
                            //
                            // TODO: mark usage of `.balanceOf(address(this))` for future checks
                            //

                            match definition_node {
                                solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                                    "\tThe {} {} in the `{}` {} contains usage of `.balanceOf(address(this))`",

                                    function_definition.visibility,

                                    if let solidity::ast::FunctionKind::Constructor = function_definition.kind {
                                        format!("{}", "constructor")
                                    } else {
                                        format!("`{}` {}", function_definition.name, function_definition.kind)
                                    },

                                    contract_definition.name,
                                    contract_definition.kind,
                                ),

                                solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                                    "\tThe `{}` modifier in the `{}` {} contains usage of `.balanceOf(address(this))`",
                                    modifier_definition.name,
                                    contract_definition.name,
                                    contract_definition.kind,
                                ),

                                _ => return Ok(())
                            }
                        }

                        _ => return Ok(()),
                    }
                }

                _ => return Ok(()),
            },

            _ => return Ok(()),
        }

        Ok(())
    }
}
