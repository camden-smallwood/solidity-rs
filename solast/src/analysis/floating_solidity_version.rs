use solidity::ast::*;
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
                        "\tL{}: Floating solidity version: {}; Consider locking before deployment",
                        
                        context.current_source_unit.source_line(context.pragma_directive.src.as_str()).unwrap(),

                        pragma_string
                    );
                }
            }
        }

        Ok(())
    }
}
