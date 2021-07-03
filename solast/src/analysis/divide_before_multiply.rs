use std::io;
use super::{AstVisitor, BinaryOperationContext};
use solidity::ast::*;

pub struct DivideBeforeMultiplyVisitor;

//
// TODO:
//   1. track variable assignments, transfering all operations that occurred
//   2. retrieve operations from function calls
//

impl AstVisitor for DivideBeforeMultiplyVisitor {
    fn visit_binary_operation<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> {
        if context.binary_operation.operator != "*" {
            return Ok(());
        }

        if let Expression::BinaryOperation(left_operation) = context.binary_operation.left_expression.as_ref() {
            if left_operation.contains_operation("/") {
                match context.definition_node {
                    ContractDefinitionNode::FunctionDefinition(function_definition) => {
                        println!(
                            "\t{} {} {} performs a multiplication on the result of a division",
                            format!("{:?}", function_definition.visibility),
                            if function_definition.name.is_empty() {
                                format!("{}", context.contract_definition.name)
                            } else {
                                format!("{}.{}", context.contract_definition.name, function_definition.name)
                            },
                            function_definition.kind
                        );
                    }

                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                        println!(
                            "\t{} {} modifier performs a multiplication on the result of a division",
                            format!("{:?}", modifier_definition.visibility),
                            if modifier_definition.name.is_empty() {
                                format!("{}", context.contract_definition.name)
                            } else {
                                format!("{}.{}", context.contract_definition.name, modifier_definition.name)
                            }
                        );
                    }

                    _ => {}
                }
            }
        }

        Ok(())
    }
}
