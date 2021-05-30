use super::AstVisitor;
use std::io;

pub struct NoSpdxIdentifierVisitor;

impl AstVisitor for NoSpdxIdentifierVisitor {
    fn visit_source_unit(
        &mut self,
        source_unit: &solidity::ast::SourceUnit
    ) -> io::Result<()> {
        if source_unit.license.is_none() {
            println!("\tSPDX license identifier not provided in source source_unit; Consider adding one before deployment");
        }

        Ok(())
    }
}
