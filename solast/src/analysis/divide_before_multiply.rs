use super::AstVisitor;
use solidity::ast::{Block, ContractDefinition, ContractDefinitionNode, NodeID, SourceUnit};
use std::{collections::HashMap, io};

struct AssignmentInfo {
    referenced_declaration: NodeID,
    operator: String,
}

struct BlockInfo {
    id: NodeID,
    blocks: Vec<BlockInfo>,
    assignments: Vec<AssignmentInfo>,
}

struct FunctionInfo {
    blocks: Vec<BlockInfo>,
}

pub struct DivideBeforeMultiplyVisitor {
    functions: HashMap<NodeID, FunctionInfo>,
}

impl Default for DivideBeforeMultiplyVisitor {
    fn default() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
}

impl AstVisitor for DivideBeforeMultiplyVisitor {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if !self.functions.contains_key(&function_definition.id) {
            self.functions
                .insert(function_definition.id, FunctionInfo { blocks: vec![] });
        }

        Ok(())
    }

    fn visit_block<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        _contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        block: &'a Block,
    ) -> io::Result<()> {
        let function_info = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                self.functions.get_mut(&function_definition.id).unwrap()
            }

            _ => return Ok(()),
        };

        let mut block_info: Option<&mut BlockInfo> = None;

        for block in blocks.iter() {
            block_info = match block_info {
                Some(block_info) => block_info.blocks.iter_mut().find(|block_info| block_info.id == block.id),
                
                None => match function_info.blocks.iter_mut().find(|block_info| block_info.id == block.id) {
                    block_info if block_info.is_some() => block_info,

                    _ => return Err(
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("function block id not found: {:?}", block.id),
                        )
                    )
                },
            };
        }

        let blocks = match block_info {
            Some(block_info) => &mut block_info.blocks,
            None => &mut function_info.blocks,
        };

        blocks.push(BlockInfo {
            id: block.id,
            blocks: vec![],
            assignments: vec![],
        });

        Ok(())
    }

    fn leave_block<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        block: &'a Block,
    ) -> io::Result<()> {
        let function_info = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                self.functions.get_mut(&function_definition.id).unwrap()
            }

            _ => return Ok(()),
        };

        let mut block_info: Option<&mut BlockInfo> = None;

        for block in blocks.iter() {
            block_info = match block_info {
                Some(block_info) => block_info.blocks.iter_mut().find(|block_info| block_info.id == block.id),
                
                None => match function_info.blocks.iter_mut().find(|block_info| block_info.id == block.id) {
                    block_info if block_info.is_some() => block_info,
                    
                    _ => return Err(
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("function block id not found: {:?}", block.id),
                        )
                    )
                }
            };
        }

        Ok(())
    }

    fn visit_assignment<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        assignment: &'a solidity::ast::Assignment,
    ) -> io::Result<()> {
        let function_info = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                self.functions.get_mut(&function_definition.id).unwrap()
            }

            _ => return Ok(())
        };

        let mut block_info: Option<&mut BlockInfo> = None;

        for block in blocks.iter() {
            block_info = match block_info {
                Some(block_info) => block_info.blocks.iter_mut().find(|block_info| block_info.id == block.id),
                
                None => match function_info.blocks.iter_mut().find(|block_info| block_info.id == block.id) {
                    block_info if block_info.is_some() => block_info,

                    _ => return Err(
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("function block id not found: {:?}", block.id),
                        )
                    )
                }
            };
        }

        let block_info = block_info.unwrap();

        if let "+=" | "-=" | "*=" | "/=" = assignment.operator.as_str() {
            for referenced_declaration in assignment.left_hand_side.referenced_declarations() {
                block_info.assignments.push(AssignmentInfo {
                    referenced_declaration,
                    operator: "*".into(),
                });
            }
        }

        Ok(())
    }

    fn visit_binary_operation<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        binary_operation: &'a solidity::ast::BinaryOperation,
    ) -> io::Result<()> {
        if binary_operation.operator != "*" {
            return Ok(());
        }

        if let solidity::ast::Expression::BinaryOperation(left_operation) = binary_operation.left_expression.as_ref() {
            if left_operation.contains_operation("/") {
                match definition_node {
                    solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => {
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

                    solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
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
