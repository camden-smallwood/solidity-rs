use super::{AstVisitor, BlockContext, FunctionDefinitionContext};
use solidity::ast::{NodeID, SourceUnit};
use std::{
    collections::{HashMap, HashSet},
    io,
};

struct BlockInfo {
    pub verified_declarations: HashSet<NodeID>,
}

struct FunctionInfo {
    pub occurance_count: usize,
}

pub struct UncheckedERC20TransferVisitor<'a> {
    source_units: &'a [SourceUnit],
    block_info: HashMap<NodeID, BlockInfo>,
    function_info: HashMap<NodeID, FunctionInfo>,
}

impl<'a> UncheckedERC20TransferVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self {
            source_units,
            block_info: HashMap::new(),
            function_info: HashMap::new(),
        }
    }
}

impl AstVisitor for UncheckedERC20TransferVisitor<'_> {
    fn visit_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> io::Result<()> {
        if !self.block_info.contains_key(&context.block.id) {
            self.block_info.insert(
                context.block.id,
                BlockInfo {
                    verified_declarations: HashSet::new(),
                },
            );
        }

        Ok(())
    }

    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if !self.function_info.contains_key(&context.function_definition.id) {
            self.function_info.insert(
                context.function_definition.id,
                FunctionInfo {
                    occurance_count: 0
                }
            );
        }

        Ok(())
    }

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        let function_info = self.function_info.get(&context.function_definition.id).unwrap();

        if function_info.occurance_count > 0 {
            println!(
                "\t{} {} {} makes {} without checking the {}, which can revert {} zero",

                format!("{:?}", context.function_definition.visibility),

                if context.function_definition.name.is_empty() {
                    format!("{}", context.contract_definition.name)
                } else {
                    format!("{}.{}", context.contract_definition.name, context.function_definition.name)
                },

                context.function_definition.kind,

                if function_info.occurance_count == 1 {
                    "an ERC-20 transfer"
                } else {
                    "ERC-20 transfers"
                },

                if function_info.occurance_count == 1 {
                    "amount"
                } else {
                    "amounts"
                },
                
                if function_info.occurance_count == 1 {
                    "if"
                } else {
                    "if any are"
                },
            );
        }

        Ok(())
    }

    fn visit_if_statement<'a, 'b>(&mut self, context: &mut super::IfStatementContext<'a, 'b>) -> io::Result<()> {
        let block = match &context.if_statement.true_body {
            solidity::ast::BlockOrStatement::Block(block) => block,
            solidity::ast::BlockOrStatement::Statement(_) => return Ok(()),
        };

        if !self.block_info.contains_key(&block.id) {
            self.block_info.insert(
                block.id,
                BlockInfo {
                    verified_declarations: HashSet::new(),
                },
            );
        }

        let block_info = self.block_info.get_mut(&block.id).unwrap();

        let mut operations = match &context.if_statement.condition {
            solidity::ast::Expression::BinaryOperation(expr) => vec![expr],
            _ => return Ok(()),
        };

        while let Some(operation) = operations.pop() {
            match operation.operator.as_str() {
                "&&" | "||" => {
                    if let solidity::ast::Expression::BinaryOperation(operation) =
                        operation.left_expression.as_ref()
                    {
                        operations.push(operation);
                    }

                    if let solidity::ast::Expression::BinaryOperation(operation) =
                        operation.right_expression.as_ref()
                    {
                        operations.push(operation);
                    }
                }

                ">" | "!=" => {
                    if let Some(&referenced_declaration) =
                        operation.left_expression.referenced_declarations().last()
                    {
                        if let solidity::ast::Expression::Literal(solidity::ast::Literal {
                            value: Some(value),
                            ..
                        }) = operation.right_expression.as_ref()
                        {
                            if value == "0"
                                && !block_info
                                    .verified_declarations
                                    .contains(&referenced_declaration)
                            {
                                block_info
                                    .verified_declarations
                                    .insert(referenced_declaration);
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        Ok(())
    }

    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        function_call: &'a solidity::ast::FunctionCall,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        for referenced_declaration in function_call.expression.referenced_declarations() {
            for source_unit in self.source_units.iter() {
                if let Some((called_contract_definition, called_function_definition)) =
                    source_unit.function_and_contract_definition(referenced_declaration)
                {
                    if let "erc20" | "ierc20" = called_contract_definition
                        .name
                        .to_ascii_lowercase()
                        .as_str()
                    {
                        if let "transfer" | "transferFrom" =
                            called_function_definition.name.as_str()
                        {
                            for block in blocks.iter() {
                                let block_info = self.block_info.get(&block.id).unwrap();

                                match function_call.arguments.last() {
                                    Some(solidity::ast::Expression::Literal(_)) => break,

                                    Some(expression)
                                        if !block_info.verified_declarations.contains(
                                            expression
                                                .referenced_declarations()
                                                .last()
                                                .unwrap_or(&0),
                                        ) =>
                                    {
                                        self.function_info
                                            .get_mut(&definition_id)
                                            .unwrap()
                                            .occurance_count += 1;
                                    }

                                    _ => {}
                                }
                            }
                        }

                        break;
                    }
                }
            }
        }

        let block = match blocks.last() {
            Some(block) => block,
            None => return Ok(()),
        };

        let block_info = self.block_info.get_mut(&block.id).unwrap();

        if let solidity::ast::Expression::Identifier(expr) = function_call.expression.as_ref() {
            if expr.name == "require" {
                let mut operations = match function_call.arguments.first().unwrap() {
                    solidity::ast::Expression::BinaryOperation(expr) => vec![expr],
                    _ => return Ok(()),
                };

                while let Some(operation) = operations.pop() {
                    match operation.operator.as_str() {
                        "&&" | "||" => {
                            if let solidity::ast::Expression::BinaryOperation(operation) =
                                operation.left_expression.as_ref()
                            {
                                operations.push(operation);
                            }

                            if let solidity::ast::Expression::BinaryOperation(operation) =
                                operation.right_expression.as_ref()
                            {
                                operations.push(operation);
                            }
                        }

                        ">" | "!=" => {
                            if let Some(&referenced_declaration) =
                                operation.left_expression.referenced_declarations().last()
                            {
                                if let solidity::ast::Expression::Literal(solidity::ast::Literal {
                                    value: Some(value),
                                    ..
                                }) = operation.right_expression.as_ref()
                                {
                                    if value == "0"
                                        && !block_info
                                            .verified_declarations
                                            .contains(&referenced_declaration)
                                    {
                                        block_info
                                            .verified_declarations
                                            .insert(referenced_declaration);
                                    }
                                }
                            }
                        }

                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
