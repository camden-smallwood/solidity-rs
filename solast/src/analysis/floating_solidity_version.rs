use super::{AstVisitor, PragmaDirectiveContext};
use std::io;

pub struct FloatingSolidityVersionVisitor;

impl AstVisitor for FloatingSolidityVersionVisitor {
    fn visit_pragma_directive<'a>(
        &mut self,
        context: &mut PragmaDirectiveContext<'a>
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
