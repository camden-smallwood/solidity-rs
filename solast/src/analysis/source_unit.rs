use super::AstVisitor;
use solidity::ast::SourceUnit;
use std::io;

pub struct SourceUnitVisitor<'a> {
    pub source_units: &'a [SourceUnit],
    pub first_file: bool,
}

impl<'a> SourceUnitVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self {
            source_units,
            first_file: true,
        }
    }
}

impl<'a> AstVisitor for SourceUnitVisitor<'a> {
    fn visit_source_unit(&mut self, source_unit: &solidity::ast::SourceUnit) -> io::Result<()> {
        if self.first_file {
            self.first_file = false;
        } else {
            println!();
        }

        println!("{}:", source_unit.absolute_path.as_ref().map(|path| path.as_str()).unwrap_or("<ABSOLUTE_PATH_NOT_SET/>"));

        Ok(())
    }
}
