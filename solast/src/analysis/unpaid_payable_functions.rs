use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct UnpaidPayableFunctionsVisitor {
    report: Rc<RefCell<Report>>,
}

impl UnpaidPayableFunctionsVisitor {
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
                "{} calls a payable function without paying: `{}`",
                contract_definition.definition_node_location(definition_node),
                expression
            ),
        );
    }
}

impl AstVisitor for UnpaidPayableFunctionsVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        match context.function_call.expression.as_ref() {
            solidity::ast::Expression::Identifier(identifier) => {
                for source_unit in context.source_units.iter() {
                    if let Some(FunctionDefinition {
                        state_mutability: StateMutability::Payable,
                        ..
                    }) = source_unit.function_definition(identifier.referenced_declaration) {
                        self.add_report_entry(
                            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                            context.contract_definition,
                            context.definition_node,
                            context.current_source_unit.source_line(context.function_call.src.as_str())?,
                            context.function_call,
                        );
                        break;
                    }
                }
            }

            solidity::ast::Expression::MemberAccess(member_access) => {
                let referenced_declaration = match member_access.referenced_declaration {
                    Some(id) => id,
                    None => return Ok(()),
                };

                for source_unit in context.source_units.iter() {
                    if let Some(FunctionDefinition {
                        state_mutability: StateMutability::Payable,
                        ..
                    }) = source_unit.function_definition(referenced_declaration) {
                        self.add_report_entry(
                            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                            context.contract_definition,
                            context.definition_node,
                            context.current_source_unit.source_line(context.function_call.src.as_str())?,
                            context.function_call,
                        );
                        break;
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }
}
