use super::{AstVisitor, SourceUnitContext};
use std::io;

pub struct NoSpdxIdentifierVisitor;

impl AstVisitor for NoSpdxIdentifierVisitor {
    fn visit_source_unit<'a>(
        &mut self,
        context: &mut SourceUnitContext<'a>
    ) -> io::Result<()> {
        if context.current_source_unit.license.is_none() {
            println!("\tSPDX license identifier not provided in source file; Consider adding one before deployment");
        }

        Ok(())
    }
}
