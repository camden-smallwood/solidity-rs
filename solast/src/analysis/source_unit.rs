use solidity::ast::*;
use std::io;

pub struct SourceUnitVisitor {
    first_file: bool,
}

impl Default for SourceUnitVisitor {
    fn default() -> Self {
        Self {
            first_file: true,
        }
    }
}

impl AstVisitor for SourceUnitVisitor {
    fn visit_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> {
        if self.first_file {
            self.first_file = false;
        } else {
            println!();
        }

        println!("{}:", context.current_source_unit.absolute_path.as_deref().unwrap_or("<ABSOLUTE_PATH_NOT_SET/>"));

        Ok(())
    }
}
