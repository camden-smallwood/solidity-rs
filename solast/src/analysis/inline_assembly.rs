use super::AstVisitor;
use solidity::ast::{ContractDefinitionNode, FunctionKind, NodeID,};
use std::{collections::HashSet, io};
use yul::{YulExpression, YulFunctionCall, YulIdentifier, YulLiteral};

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
    fn visit_inline_assembly<'a, 'b>(&mut self, context: &mut super::InlineAssemblyContext<'a, 'b>) -> io::Result<()> {
        let function_definition = match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(()),
        };

        if !self.reported_functions.contains(&function_definition.id) {
            self.reported_functions.insert(function_definition.id);

            println!(
                "\t{} {} {} contains inline assembly usage",
                format!("{:?}", function_definition.visibility),
                if let FunctionKind::Constructor = function_definition.kind {
                    format!("{}", context.contract_definition.name)
                } else {
                    format!("{}.{}", context.contract_definition.name, function_definition.name)
                },
                function_definition.kind,
            );
        }

        Ok(())
    }

    fn visit_yul_function_call<'a, 'b, 'c>(&mut self, context: &mut super::YulFunctionCallContext<'a, 'b, 'c>) -> io::Result<()> {
        let function_definition = match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(()),
        };

        match context.yul_function_call.function_name.name.as_str() {
            "mload" => {
                let value = match context.yul_function_call.arguments.first() {
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
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
                        },
                        function_definition.kind
                    );
                }
            }

            "calldatacopy" => {
                let arguments = match context.yul_function_call.arguments.iter().nth(2) {
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
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
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
