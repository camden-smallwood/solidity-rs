use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct FloatingSolidityVersionVisitor {
    report: Rc<RefCell<Report>>,
}

impl FloatingSolidityVersionVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }
}

impl AstVisitor for FloatingSolidityVersionVisitor {
    fn visit_pragma_directive<'a>(
        &mut self,
        context: &mut PragmaDirectiveContext<'a>,
    ) -> io::Result<()> {
        if let Some(literal) = context.pragma_directive.literals.first() {
            if literal == "solidity" {
                let mut pragma_string = String::new();
                let mut floating = false;

                for literal in context.pragma_directive.literals.iter().skip(1) {
                    if let "^" | ">" | ">=" | "<" | "<=" = literal.as_str() {
                        if !pragma_string.is_empty() {
                            pragma_string.push(' ');
                        }

                        floating = true;
                    }

                    pragma_string.push_str(literal);
                }

                if floating {
                    self.report.borrow_mut().add_entry(
                        context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                        Some(context.current_source_unit.source_line(context.pragma_directive.src.as_str())?),
                        format!("Floating solidity version: {pragma_string}; Consider locking before deployment"),
                    );
                }
            }
        }

        Ok(())
    }
}
