use solidity::ast::*;
use std::io;
use yul::ast::*;

pub struct InlineAssemblyVisitor;

impl InlineAssemblyVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        description: &str
    ) {
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} contains {}",
    
                source_line,
    
                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    format!("{}", "constructor")
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },
    
                contract_definition.name,
                contract_definition.kind,
    
                description
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The `{}` modifier in the `{}` {} contains {}",

                source_line,

                modifier_definition.name,

                contract_definition.name,
                contract_definition.kind,
    
                description
            ),

            _ => {}
        }
    }
}

impl AstVisitor for InlineAssemblyVisitor {
    fn visit_inline_assembly<'a, 'b>(&mut self, context: &mut InlineAssemblyContext<'a, 'b>) -> io::Result<()> {
        self.print_message(
            context.contract_definition,
            context.definition_node,
            context.current_source_unit.source_line(context.inline_assembly.src.as_str())?,
            "inline assembly usage"
        );

        Ok(())
    }

    fn visit_yul_function_call<'a, 'b, 'c>(&mut self, context: &mut YulFunctionCallContext<'a, 'b, 'c>) -> io::Result<()> {
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
                    self.print_message(
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(context.inline_assembly.src.as_str())?,
                        "inline assembly which loads the free memory pointer"
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
                    self.print_message(
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(context.inline_assembly.src.as_str())?,
                        "inline assembly which copies arbitrary function arguments"
                    );
                }
            }

            _ => {}
        }

        Ok(())
    }
}
