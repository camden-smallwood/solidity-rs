use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, rc::Rc};

pub struct ArrayAssignmentVisitor {
    report: Rc<RefCell<Report>>,
}

impl ArrayAssignmentVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        index_access: &IndexAccess,
        operator: &str,
        expression: &Expression,
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} contains an inefficient array assignment which can be optimized to `{} {}= {};`",
                contract_definition.definition_node_location(definition_node),
                index_access,
                operator,
                expression,
            ),
        );
    }
}

impl AstVisitor for ArrayAssignmentVisitor {
    fn visit_assignment<'a, 'b>(&mut self, context: &mut AssignmentContext<'a, 'b>) -> std::io::Result<()> {
        if context.assignment.operator != "=" {
            return Ok(());
        }

        let index_access = match context.assignment.left_hand_side.as_ref() {
            Expression::IndexAccess(index_access) => index_access,
            _ => return Ok(()),
        };

        let binary_operation = match context.assignment.right_hand_side.as_ref() {
            Expression::BinaryOperation(binary_operation) => binary_operation,
            _ => return Ok(()),
        };

        if !matches!(binary_operation.operator.as_str(), "+" | "-" | "*" | "/" | "%" | "<<" | ">>" | "&" | "|" | "^") {
            return Ok(());
        }

        let index_access2 = match binary_operation.left_expression.as_ref() {
            Expression::IndexAccess(index_access2) => index_access2,
            _ => return Ok(()),
        };

        if index_access.base_expression == index_access2.base_expression {
            self.add_report_entry(
                context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.assignment.src.as_str())?,
                index_access,
                binary_operation.operator.as_str(),
                binary_operation.right_expression.as_ref(),
            );
        }

        Ok(())
    }
}
