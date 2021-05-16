use super::AstVisitor;
use std::io;

pub struct FloatingSolidityVersionVisitor;

impl AstVisitor for FloatingSolidityVersionVisitor {
    fn visit_pragma_directive(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        pragma_directive: &solidity::ast::PragmaDirective,
    ) -> io::Result<()> {
        if let Some(literal) = pragma_directive.literals.first() {
            if literal == "solidity" {
                let mut pragma_string = String::new();
                let mut floating = false;

                for literal in pragma_directive.literals.iter().skip(1) {
                    if let "^" | ">" | ">=" | "<" | "<=" = literal.as_str() {
                        if !pragma_string.is_empty() {
                            pragma_string.push(' ');
                        }

                        floating = true;
                    }

                    pragma_string.push_str(literal);
                }

                if floating {
                    println!(
                        "\tFloating solidity version: {}; Consider locking before deployment",
                        pragma_string
                    );
                }
            }
        }

        Ok(())
    }
}
