use std::io;
use super::AstVisitor;
use solidity::ast::{BinaryOperation, Block, ContractDefinition, ContractDefinitionNode, Expression, SourceUnit, Statement};

pub struct DivideBeforeMultiplyVisitor;

impl AstVisitor for DivideBeforeMultiplyVisitor {
    fn visit_binary_operation<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        binary_operation: &'a BinaryOperation,
    ) -> io::Result<()> {
        if binary_operation.operator != "*" {
            return Ok(());
        }

        if let Expression::BinaryOperation(left_operation) = binary_operation.left_expression.as_ref() {
            if left_operation.contains_operation("/") {
                match definition_node {
                    ContractDefinitionNode::FunctionDefinition(function_definition) => {
                        println!(
                            "\t{} {} {} performs a multiplication on the result of a division",
                            format!("{:?}", function_definition.visibility),
                            if function_definition.name.is_empty() {
                                format!("{}", contract_definition.name)
                            } else {
                                format!("{}.{}", contract_definition.name, function_definition.name)
                            },
                            format!("{:?}", function_definition.kind).to_lowercase()
                        );
                    }

                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                        println!(
                            "\t{} {} modifier performs a multiplication on the result of a division",
                            format!("{:?}", modifier_definition.visibility),
                            if modifier_definition.name.is_empty() {
                                format!("{}", contract_definition.name)
                            } else {
                                format!("{}.{}", contract_definition.name, modifier_definition.name)
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
