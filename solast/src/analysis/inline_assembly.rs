use super::AstVisitor;
use solidity::ast::{
    Block, ContractDefinition, ContractDefinitionNode, FunctionKind, NodeID, SourceUnit, Statement,
};
use std::{collections::HashSet, io};
use yul::{InlineAssembly, YulBlock, YulExpression, YulFunctionCall, YulIdentifier, YulLiteral, YulStatement};

pub struct InlineAssemblyVisitor {
    reported_functions: HashSet<NodeID>,
}

impl Default for InlineAssemblyVisitor {
    fn default() -> Self {
        Self {
            reported_functions: HashSet::new(),
        }
    }
}

impl AstVisitor for InlineAssemblyVisitor {
    fn visit_inline_assembly<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: &'a Statement,
        _inline_assembly: &'a InlineAssembly,
    ) -> io::Result<()> {
        let function_definition = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(()),
        };

        if !self.reported_functions.contains(&function_definition.id) {
            self.reported_functions.insert(function_definition.id);

            println!(
                "\t{} {} {} contains inline assembly usage",
                format!("{:?}", function_definition.visibility),
                if let FunctionKind::Constructor = function_definition.kind {
                    format!("{}", contract_definition.name)
                } else {
                    format!("{}.{}", contract_definition.name, function_definition.name)
                },
                function_definition.kind,
            );
        }

        Ok(())
    }

    fn visit_yul_function_call<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: &'a Statement,
        _inline_assembly: &'a InlineAssembly,
        _yul_blocks: &mut Vec<&'a YulBlock>,
        _yul_statement: Option<&'a YulStatement>,
        _yul_expression: &'a YulExpression,
        yul_function_call: &'a YulFunctionCall,
    ) -> io::Result<()> {
        let function_definition = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(()),
        };

        match yul_function_call.function_name.name.as_str() {
            "mload" => {
                let value = match yul_function_call.arguments.first() {
                    Some(
                        YulExpression::YulLiteral(YulLiteral {
                            value: Some(value),
                            ..
                        })
                        | YulExpression::YulLiteral(YulLiteral {
                            hex_value: Some(value),
                            ..
                        })
                    ) => value,

                    _ => return Ok(())
                };

                if let Ok(0x40) = if value.starts_with("0x") {
                    i64::from_str_radix(value.trim_start_matches("0x"), 16)
                } else {
                    value.parse()
                } {
                    println!(
                        "\t{} {} {} contains inline assembly which loads the free memory pointer",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.kind == FunctionKind::Constructor {
                            format!("{}", contract_definition.name)
                        } else {
                            format!("{}.{}", contract_definition.name, function_definition.name)
                        },
                        function_definition.kind
                    );
                }
            }

            "calldatacopy" => {
                let arguments = match yul_function_call.arguments.iter().nth(2) {
                    Some(YulExpression::YulFunctionCall(YulFunctionCall {
                        function_name: YulIdentifier { name },
                        arguments,
                    })) if name == "sub" => arguments,

                    _ => return Ok(())
                };

                match arguments.iter().nth(0) {
                    Some(YulExpression::YulFunctionCall(YulFunctionCall {
                        function_name: YulIdentifier { name },
                        ..
                    })) if name == "calldatasize" => {}

                    _ => return Ok(())
                }

                let value = match arguments.iter().nth(1) {
                    Some(
                        YulExpression::YulLiteral(YulLiteral {
                            value: Some(value),
                            ..
                        })
                        | YulExpression::YulLiteral(YulLiteral {
                            hex_value: Some(value),
                            ..
                        })
                    ) => value,

                    _ => return Ok(())
                };
                
                if let Ok(0x4) = if value.starts_with("0x") {
                    i64::from_str_radix(value.trim_start_matches("0x"), 16)
                } else {
                    value.parse()
                } {
                    println!(
                        "\t{} {} {} contains inline assembly which copies arbitrary function arguments",
                        format!("{:?}", function_definition.visibility),
                        if function_definition.kind == FunctionKind::Constructor {
                            format!("{}", contract_definition.name)
                        } else {
                            format!("{}.{}", contract_definition.name, function_definition.name)
                        },
                        function_definition.kind
                    );
                }
            }

            _ => {}
        }

        Ok(())
    }
}
