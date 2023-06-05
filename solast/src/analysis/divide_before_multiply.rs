use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct DivideBeforeMultiplyVisitor {
    report: Rc<RefCell<Report>>,
}

impl DivideBeforeMultiplyVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }
}

//
// TODO:
//   1. track variable assignments, transfering all operations that occurred
//   2. retrieve operations from function calls
//

impl AstVisitor for DivideBeforeMultiplyVisitor {
    fn visit_binary_operation<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> {
        if context.binary_operation.operator != "*" {
            return Ok(())
        }

        if let Expression::BinaryOperation(left_operation) = context.binary_operation.left_expression.as_ref() {
            if left_operation.contains_operation("/") {
                self.report.borrow_mut().add_entry(
                    context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                    Some(context.current_source_unit.source_line(context.binary_operation.src.as_str())?),
                    format!(
                        "{} performs a multiplication on the result of a division",
                        context.contract_definition.definition_node_location(context.definition_node),
                    ),
                );
            }
        }

        Ok(())
    }
}
