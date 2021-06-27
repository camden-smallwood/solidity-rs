use super::{AstVisitor, SourceUnitContext};
use std::io;

pub struct UnnecessaryPragmasVisitor;

impl AstVisitor for UnnecessaryPragmasVisitor {
    fn visit_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> {
        let mut solidity: Vec<&str> = vec![];
        let mut abicoder: Vec<&str> = vec![];

        for pragma_directive in context.current_source_unit.pragma_directives() {
            match pragma_directive.literals.first() {
                Some(literal) if literal == "solidity" => {
                    assert!(solidity.is_empty());
                    solidity.extend(pragma_directive.literals.iter().skip(1).map(String::as_str));
                }

                Some(literal) if literal == "abicoder" => {
                    assert!(abicoder.is_empty());
                    abicoder.extend(pragma_directive.literals.iter().skip(1).map(String::as_str));
                }

                _ => {}
            }
        }

        let mut op = None;
        let mut lower: Option<f32> = None;
        let mut upper: Option<f32> = None;

        for literal in solidity {
            match literal {
                "^" | ">" | ">=" | "<=" | "<" => op = Some(literal),

                _ if !literal.starts_with(".") => {
                    match op {
                        None | Some("=" | "^" | ">" | ">=") => {
                            lower = Some(literal.parse().unwrap_or(0.0));

                            if let Some(f) = lower {
                                if f == 0.0 {
                                    lower = None;
                                }
                            }
                        }

                        Some("<" | "<=") => {
                            upper = Some(literal.parse().unwrap_or(0.0));

                            if let Some(f) = upper {
                                if f == 0.0 {
                                    upper = None;
                                }
                            }
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        if lower.is_some() && upper.is_none() {
            upper = Some(lower.unwrap() + 0.1);
        }
        
        if lower.is_none() && upper.is_some() {
            lower = Some(upper.unwrap() - 0.1);
        }

        if abicoder.contains(&"v2") {
            if let Some(lower) = lower {
                if lower >= 0.8 {
                    println!("\tUnnecessary specification of `pragma abicoder v2`, which is enabled in Solidity v0.8.0 and above");
                }
            }
        }

        return Ok(())
    }
}
