use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, rc::Rc};

//
// TODO:
// * Check if a state variable is accessed multiple times in a block without any changes being made
// * Check if member access is a local variable bound to an array state variable
//

pub struct RedundantStateVariableAccessVisitor {
    report: Rc<RefCell<Report>>,
}

impl RedundantStateVariableAccessVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        message: &str,
        expression: &dyn std::fmt::Display
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} contains {} which redundantly accesses storage: `{}`",
                contract_definition.definition_node_location(definition_node),
                message,
                expression
            ),
        );
    }
}

impl AstVisitor for RedundantStateVariableAccessVisitor {
    fn visit_for_statement<'a, 'b>(&mut self, context: &mut ForStatementContext<'a, 'b>) -> std::io::Result<()> {
        //
        // Check if the for statement's condition directly references a state variable
        //

        let condition = match context.for_statement.condition.as_ref() {
            Some(condition) => condition,
            None => return Ok(())
        };

        for id in condition.referenced_declarations() {
            if context.contract_definition.hierarchy_contains_state_variable(context.source_units, id) {
                self.add_report_entry(
                    context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                    context.contract_definition,
                    context.definition_node,
                    condition.source_line(context.current_source_unit)?,
                    "a for statement with a condition",
                    condition
                );
                return Ok(())
            }
        }

        Ok(())
    }

    fn visit_while_statement<'a, 'b>(&mut self, context: &mut WhileStatementContext<'a, 'b>) -> std::io::Result<()> {
        //
        // Check if the while statement's condition directly references a state variable
        //

        for id in context.while_statement.condition.referenced_declarations() {
            if context.contract_definition.hierarchy_contains_state_variable(context.source_units, id) {
                self.add_report_entry(
                    context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                    context.contract_definition,
                    context.definition_node,
                    context.while_statement.condition.source_line(context.current_source_unit)?,
                    "a while statement with a condition",
                    &context.while_statement.condition
                );
                return Ok(())
            }
        }

        Ok(())
    }
}
