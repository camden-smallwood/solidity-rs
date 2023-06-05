use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct SafeERC20FunctionsVisitor {
    report: Rc<RefCell<Report>>,
}

impl SafeERC20FunctionsVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        unsafe_name: &str,
        safe_name: &str
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} uses `ERC20.{}` instead of `SafeERC20.{}`",
                contract_definition.definition_node_location(definition_node),
                unsafe_name,
                safe_name,
            ),
        );
    }
}

impl AstVisitor for SafeERC20FunctionsVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if context.contract_definition.name == "SafeERC20" {
            return Ok(())
        }

        for referenced_declaration in context.function_call.expression.referenced_declarations() {
            for source_unit in context.source_units.iter() {
                let (called_contract_definition, called_function_definition) = match source_unit.function_and_contract_definition(referenced_declaration) {
                    Some((contract_definition, function_definition)) => (contract_definition, function_definition),
                    None => continue
                };
            
                match called_contract_definition.name.to_ascii_lowercase().as_str() {
                    "erc20" | "ierc20" | "erc20interface" => {}
                    _ => return Ok(())
                }

                let (unsafe_name, safe_name) = match called_function_definition.name.as_str() {
                    "transfer" => ("transfer", "safeTransfer"),
                    "transferFrom" => ("transferFrom", "safeTransferFrom"),
                    "approve" => ("approve", "safeApprove"),
                    _ => continue,
                };
                
                self.add_report_entry(
                    context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                    context.contract_definition,
                    context.definition_node,
                    context.current_source_unit.source_line(context.function_call.src.as_str())?,
                    unsafe_name,
                    safe_name
                );

                break;
            }
        }

        Ok(())
    }
}
