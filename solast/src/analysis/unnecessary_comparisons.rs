use std::io;

use solidity::ast::{BinaryOperation, Block, ContractDefinition, ContractDefinitionNode, Expression, FunctionKind, Literal, SourceUnit, Statement};

use super::AstVisitor;

pub struct UnnecessaryComparisonsVisitor;

impl AstVisitor for UnnecessaryComparisonsVisitor {
    //
    // TODO:
    // * uint8 < 256
    //

    fn visit_binary_operation<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        binary_operation: &'a BinaryOperation,
    ) -> io::Result<()> {
        match binary_operation.operator.as_str() {
            ">=" => {
                let type_name = match binary_operation.left_expression.as_ref() {
                    Expression::Identifier(expr) => expr.type_descriptions.type_string.as_ref(),
                    Expression::UnaryOperation(expr) => expr.type_descriptions.type_string.as_ref(),
                    Expression::Conditional(expr) => expr.type_descriptions.type_string.as_ref(),
                    Expression::Assignment(expr) => expr.type_descriptions.type_string.as_ref(),
                    Expression::FunctionCall(expr) => expr.type_descriptions.type_string.as_ref(),
                    Expression::FunctionCallOptions(expr) => expr.type_descriptions.type_string.as_ref(),
                    Expression::IndexAccess(expr) => expr.type_descriptions.type_string.as_ref(),
                    Expression::IndexRangeAccess(expr) => expr.type_descriptions.type_string.as_ref(),
                    Expression::MemberAccess(expr) => expr.type_descriptions.type_string.as_ref(),
                    _ => return Ok(())
                }
                .clone()
                .map(String::as_str)
                .unwrap_or("");

                if !type_name.starts_with("uint") {
                    return Ok(())
                }
            
                if let Expression::Literal(
                    Literal { value: Some(value), .. } |
                    Literal { hex_value: Some(value), .. }
                ) = binary_operation.right_expression.as_ref() {
                    if let Ok(0) = if value.starts_with("0x") {
                        i64::from_str_radix(value.trim_start_matches("0x"), 16)
                    } else {
                        value.parse()
                    } {
                        match definition_node {
                            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                                "\t{} {} {} contains a redundant comparison: {}",
                                format!("{:?}", function_definition.visibility),
                                if function_definition.kind == FunctionKind::Constructor {
                                    format!("{}", contract_definition.name)
                                } else {
                                    format!("{}.{}", contract_definition.name, function_definition.name)
                                },
                                function_definition.kind,
                                binary_operation
                            ),

                            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                                "\t{} {} modifier contains a redundant comparison: {}",
                                format!("{:?}", modifier_definition.visibility),
                                format!("{}.{}", contract_definition.name, modifier_definition.name),
                                binary_operation
                            ),

                            _ => ()
                        }
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }
}
