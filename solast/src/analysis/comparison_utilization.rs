use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct ComparisonUtilizationVisitor {
    report: Rc<RefCell<Report>>,
}

impl ComparisonUtilizationVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }
}

impl AstVisitor for ComparisonUtilizationVisitor {
    fn visit_if_statement<'a, 'b>(&mut self, _context: &mut IfStatementContext<'a, 'b>) -> io::Result<()> {
        //
        // TODO:
        //
        // Scenario:
        //   if (x > 0) {
        //     doSomething(y);
        //   } else {
        //     doSomething(z);
        //   }
        //
        // Description:
        //   Verify `x` is utilized within the `true` or `false` blocks.
        //   Unless `y` or `z` is bound to `x`, then `x` goes unutilized, which can be unintentional.
        //

        Ok(())
    }
}
