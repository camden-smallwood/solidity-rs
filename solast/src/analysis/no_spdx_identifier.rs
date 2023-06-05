use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct NoSpdxIdentifierVisitor {
    report: Rc<RefCell<Report>>,
}

impl NoSpdxIdentifierVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }
}

impl AstVisitor for NoSpdxIdentifierVisitor {
    fn visit_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> {
        if context.current_source_unit.license.is_none() {
            self.report.borrow_mut().add_entry(
                context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                None,
                "\tSPDX license identifier not provided in source file; Consider adding one before deployment"
            );
        }

        Ok(())
    }
}
