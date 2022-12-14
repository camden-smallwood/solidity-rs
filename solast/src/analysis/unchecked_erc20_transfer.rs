use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::{collections::{HashMap, HashSet}, io};

struct BlockInfo {
    verified_declarations: HashSet<NodeID>,
}

struct FunctionInfo {
    occurance_count: usize,
}

#[derive(Default)]
pub struct UncheckedERC20TransferVisitor {
    block_info: HashMap<NodeID, BlockInfo>,
    function_info: HashMap<NodeID, FunctionInfo>,
}

impl AstVisitor for UncheckedERC20TransferVisitor {
    fn visit_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> io::Result<()> {
        self.block_info.entry(context.block.id).or_insert_with(|| BlockInfo {
            verified_declarations: HashSet::new(),
        });

        Ok(())
    }

    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        self.function_info.entry(context.function_definition.id).or_insert_with(|| FunctionInfo {
            occurance_count: 0
        });

        Ok(())
    }

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        let function_info = self.function_info.get(&context.function_definition.id).unwrap();

        if function_info.occurance_count > 0 {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tL{}: The {} {} in the `{}` {} makes {} without checking the {}, which can revert {} zero",

                    context.current_source_unit.source_line(context.function_definition.src.as_str())?,

                    function_definition.visibility,

                    if let FunctionKind::Constructor = function_definition.kind {
                        "constructor".to_string()
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },
        
                    context.contract_definition.name,
                    context.contract_definition.kind,

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
                ),
                
                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tL{}: The `{}` modifier in the `{}` {} makes {} without checking the {}, which can revert {} zero",

                    context.current_source_unit.source_line(context.function_definition.src.as_str())?,

                    modifier_definition.name,
        
                    context.contract_definition.name,
                    context.contract_definition.kind,

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
                ),
                
                _ => ()
            }
        }

        Ok(())
    }

    fn visit_if_statement<'a, 'b>(&mut self, context: &mut IfStatementContext<'a, 'b>) -> io::Result<()> {
        let block = match &context.if_statement.true_body {
            solidity::ast::BlockOrStatement::Block(block) => block,
            solidity::ast::BlockOrStatement::Statement(_) => return Ok(()),
        };

        let block_info = self.block_info.entry(block.id).or_insert_with(|| BlockInfo {
            verified_declarations: HashSet::new(),
        });

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

    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        let definition_id = match context.definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        for referenced_declaration in context.function_call.expression.referenced_declarations() {
            for source_unit in context.source_units.iter() {
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
                            for block in context.blocks.iter() {
                                let block_info = self.block_info.get(&block.id).unwrap();

                                match context.function_call.arguments.last() {
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

        let block = match context.blocks.last() {
            Some(block) => block,
            None => return Ok(()),
        };

        let block_info = self.block_info.get_mut(&block.id).unwrap();

        if let solidity::ast::Expression::Identifier(expr) = context.function_call.expression.as_ref() {
            if expr.name == "require" {
                let mut operations = match context.function_call.arguments.first().unwrap() {
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
