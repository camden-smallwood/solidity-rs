use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct RequireWithoutMessageVisitor {
    report: Rc<RefCell<Report>>,
}

impl RequireWithoutMessageVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }
}

impl AstVisitor for RequireWithoutMessageVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if let Expression::Identifier(Identifier { name, .. }) = context.function_call.expression.as_ref() {
            if name == "require" && context.function_call.arguments.len() < 2 {
                self.report.borrow_mut().add_entry(
                    context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                    Some(context.current_source_unit.source_line(context.function_call.src.as_str())?),
                    format!(
                        "{} contains a requirement without a message: `{}`",
                        context.contract_definition.definition_node_location(context.definition_node),
                        context.function_call
                    ),
                );
            }
        }

        Ok(())
    }
}
