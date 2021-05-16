use super::AstVisitor;
use crate::truffle;
use std::io;

pub struct SourceUnitVisitor<'a> {
    pub files: &'a [truffle::File],
    pub first_file: bool,
}

impl<'a> SourceUnitVisitor<'a> {
    pub fn new(files: &'a [truffle::File]) -> Self {
        Self {
            files,
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

        println!("{}:", source_unit.absolute_path);

        Ok(())
    }
}
