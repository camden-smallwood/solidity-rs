use super::AstVisitor;
use crate::truffle;
use solidity::ast::NodeID;
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
    files: &'a [truffle::File],
    block_info: HashMap<NodeID, BlockInfo>,
    function_info: HashMap<NodeID, FunctionInfo>,
}

impl<'a> UncheckedERC20TransferVisitor<'a> {
    pub fn new(files: &'a [truffle::File]) -> Self {
        Self {
            files,
            block_info: HashMap::new(),
            function_info: HashMap::new(),
        }
    }
}

impl AstVisitor for UncheckedERC20TransferVisitor<'_> {
    fn visit_block<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _function_definition: &'a solidity::ast::FunctionDefinition,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        block: &'a solidity::ast::Block,
    ) -> io::Result<()> {
        if !self.block_info.contains_key(&block.id) {
            self.block_info.insert(
                block.id,
                BlockInfo {
                    verified_declarations: HashSet::new(),
                },
            );
        }

        Ok(())
    }

    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if !self.function_info.contains_key(&function_definition.id) {
            self.function_info
                .insert(function_definition.id, FunctionInfo { occurance_count: 0 });
        }

        Ok(())
    }

    fn leave_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        let function_info = self.function_info.get(&function_definition.id).unwrap();

        if function_info.occurance_count > 0 {
            println!(
                "\t{} {} {} makes {} without checking the {}, which can revert {} zero",
                format!("{:?}", function_definition.visibility),
                if function_definition.name.is_empty() {
                    format!("{}", contract_definition.name)
                } else {
                    format!("{}.{}", contract_definition.name, function_definition.name)
                },
                format!("{:?}", function_definition.kind).to_lowercase(),
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

    fn visit_if_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _function_definition: &'a solidity::ast::FunctionDefinition,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        if_statement: &'a solidity::ast::IfStatement,
    ) -> io::Result<()> {
        let block = match &if_statement.true_body {
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

        let mut operations = match &if_statement.condition {
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
        function_definition: Option<&'a solidity::ast::FunctionDefinition>,
        blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        function_call: &'a solidity::ast::FunctionCall,
    ) -> io::Result<()> {
        let function_definition = match function_definition {
            Some(function_definition) => function_definition,
            None => return Ok(()),
        };

        for referenced_declaration in function_call.expression.referenced_declarations() {
            for file in self.files.iter() {
                if let Some((called_contract_definition, called_function_definition)) =
                    file.function_and_contract_definition(referenced_declaration)
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
                                            .get_mut(&function_definition.id)
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
