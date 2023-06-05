use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};
use yul::ast::*;

pub struct InlineAssemblyVisitor {
    report: Rc<RefCell<Report>>,
}

impl InlineAssemblyVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        description: &str
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} contains {}",
                contract_definition.definition_node_location(definition_node),
                description
            ),
        );
    }
}

impl AstVisitor for InlineAssemblyVisitor {
    fn visit_inline_assembly<'a, 'b>(&mut self, context: &mut InlineAssemblyContext<'a, 'b>) -> io::Result<()> {
        self.add_report_entry(
            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
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
                    self.add_report_entry(
                        context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(context.inline_assembly.src.as_str())?,
                        "inline assembly which loads the free memory pointer"
                    );
                }
            }

            "calldatacopy" => {
                let arguments = match context.yul_function_call.arguments.get(2) {
                    Some(YulExpression::YulFunctionCall(YulFunctionCall {
                        function_name: YulIdentifier { name },
                        arguments,
                    })) if name == "sub" => arguments,

                    _ => return Ok(())
                };

                match arguments.get(0) {
                    Some(YulExpression::YulFunctionCall(YulFunctionCall {
                        function_name: YulIdentifier { name },
                        ..
                    })) if name == "calldatasize" => {}

                    _ => return Ok(())
                }

                let value = match arguments.get(1) {
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
                    self.add_report_entry(
                        context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
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
