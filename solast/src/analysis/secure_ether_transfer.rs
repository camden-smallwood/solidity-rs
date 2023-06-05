use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct SecureEtherTransferVisitor {
    report: Rc<RefCell<Report>>,
}

impl SecureEtherTransferVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        expression: &dyn std::fmt::Display
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} ignores the Secure-Ether-Transfer pattern: `{}`",
                contract_definition.definition_node_location(definition_node),
                expression
            ),
        );
    }
}

impl AstVisitor for SecureEtherTransferVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if let Expression::MemberAccess(member_access) = context.function_call.expression.as_ref() {
            if let Some(TypeDescriptions { type_string: Some(type_string), .. }) = member_access.expression.as_ref().type_descriptions() {
                match type_string.as_str() {
                    "address" | "address payable" => {}
                    _ => return Ok(())
                }
            }

            match member_access.member_name.as_str() {
                "transfer" | "send" => {}
                _ => return Ok(())
            }
            
            if member_access.referenced_declaration.is_none() || member_access.referenced_declaration.map(|id| id == 0).unwrap_or(false) {
                self.add_report_entry(
                    context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                    context.contract_definition,
                    context.definition_node,
                    context.current_source_unit.source_line(context.function_call.src.as_str())?,
                    context.function_call
                );
            }
        }

        Ok(())
    }

    fn visit_function_call_options<'a, 'b>(&mut self, context: &mut FunctionCallOptionsContext<'a, 'b>) -> io::Result<()> {
        if let Expression::MemberAccess(member_access) = context.function_call_options.expression.as_ref() {
            if let Some(TypeDescriptions { type_string: Some(type_string), .. }) = member_access.expression.as_ref().type_descriptions() {
                match type_string.as_str() {
                    "address" | "address payable" => {}
                    _ => return Ok(())
                }
            }

            match member_access.member_name.as_str() {
                "transfer" | "send" => {}
                _ => return Ok(())
            }
            
            if member_access.referenced_declaration.is_none() || member_access.referenced_declaration.map(|id| id == 0).unwrap_or(false) {
                self.add_report_entry(
                    context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                    context.contract_definition,
                    context.definition_node,
                    context.current_source_unit.source_line(context.function_call_options.src.as_str())?,
                    context.function_call_options
                );
            }
        }

        Ok(())
    }
}
