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
                if let Some(YulExpression::YulLiteral(yul_literal)) =
                    yul_function_call.arguments.first()
                {
                    let mut is_free_ptr = false;

                    if let Some(Ok(0x40)) = yul_literal.value.as_ref().map(|s| {
                        if s.starts_with("0x") {
                            i64::from_str_radix(s.trim_start_matches("0x"), 16)
                        } else {
                            s.parse()
                        }
                    }) {
                        is_free_ptr = true;
                    }

                    if let Some(Ok(0x40)) = yul_literal.hex_value.as_ref().map(|s| {
                        if s.starts_with("0x") {
                            i64::from_str_radix(s.trim_start_matches("0x"), 16)
                        } else {
                            s.parse()
                        }
                    }) {
                        is_free_ptr = true;
                    }

                    if is_free_ptr {
                        println!(
                            "\t{} {} {} contains inline assembly which loads the free memory pointer",
                            format!("{:?}", function_definition.visibility),
                            if function_definition.kind == FunctionKind::Constructor {
                                format!("{}", contract_definition.name)
                            } else {
                                format!("{}.{}", contract_definition.name, function_definition.name)
                            },
                            format!("{:?}", function_definition.kind).to_lowercase()
                        );
                    }
                }
            }

            "calldatacopy" => {
                if let Some(YulExpression::YulFunctionCall(YulFunctionCall {
                    function_name: YulIdentifier {
                        name: function_name
                    },
                    arguments,
                })) = yul_function_call.arguments.iter().nth(2) {
                    if function_name == "sub" {
                        if let Some(YulExpression::YulFunctionCall(YulFunctionCall {
                            function_name: YulIdentifier {
                                name: function_name
                            },
                            ..
                        })) = arguments.iter().nth(0) {
                            if function_name == "calldatasize" {
                                if let Some(YulExpression::YulLiteral(YulLiteral {
                                    value,
                                    hex_value,
                                    ..
                                })) = arguments.iter().nth(1) {
                                    let mut copies_arbitrary_arguments = false;

                                    if let Some(Ok(0x4)) = value.as_ref().map(|s| {
                                        if s.starts_with("0x") {
                                            i64::from_str_radix(s.trim_start_matches("0x"), 16)
                                        } else {
                                            s.parse()
                                        }
                                    }) {
                                        copies_arbitrary_arguments = true;
                                    }

                                    if let Some(Ok(0x4)) = hex_value.as_ref().map(|s| {
                                        if s.starts_with("0x") {
                                            i64::from_str_radix(s.trim_start_matches("0x"), 16)
                                        } else {
                                            s.parse()
                                        }
                                    }) {
                                        copies_arbitrary_arguments = true;
                                    }

                                    if copies_arbitrary_arguments {
                                        println!(
                                            "\t{} {} {} contains inline assembly which copies arbitrary function arguments",
                                            format!("{:?}", function_definition.visibility),
                                            if function_definition.kind == FunctionKind::Constructor {
                                                format!("{}", contract_definition.name)
                                            } else {
                                                format!("{}.{}", contract_definition.name, function_definition.name)
                                            },
                                            format!("{:?}", function_definition.kind).to_lowercase()
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }
}
