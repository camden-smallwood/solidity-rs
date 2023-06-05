use crate::report::Report;
use solidity::ast::*;
use std::{io, cell::RefCell, rc::Rc};

pub struct LargeLiteralsVisitor {
    report: Rc<RefCell<Report>>,
}

impl LargeLiteralsVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }
}

impl AstVisitor for LargeLiteralsVisitor {
    fn visit_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> {
        if let Some(value) = context.literal.value.as_ref() {
            let n = value.len();

            if value.chars().all(char::is_numeric) && (n > 6) && ((n % 3) != 0) {
                self.report.borrow_mut().add_entry(
                    context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                    Some(context.current_source_unit.source_line(context.literal.src.as_str())?),
                    format!(
                        "{} contains a large literal, which may be difficult to read: `{}`",
                        context.contract_definition.definition_node_location(context.definition_node),
                        context.literal
                    ),
                );
            }
        }

        Ok(())
    }
}
