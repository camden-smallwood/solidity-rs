use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct UnnecessaryPragmasVisitor {
    report: Rc<RefCell<Report>>,
}

impl UnnecessaryPragmasVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn check_pragma_directives(
        &mut self,
        source_unit_path: String,
        source_line: Option<usize>,
        solidity: &mut Vec<&str>,
        abicoder: &mut Vec<&str>,
    ) {
        let mut op = None;
        let mut lower: Option<f32> = None;
        let mut upper: Option<f32> = None;

        for &literal in solidity.iter() {
            match literal {
                "^" | ">" | ">=" | "<=" | "<" => op = Some(literal),

                _ if !literal.starts_with('.') => {
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

        if let (Some(lower), None) = (lower, upper) {
            upper = Some(lower + 0.1);
        }

        if let (None, Some(upper)) = (lower, upper) {
            lower = Some(upper - 0.1);
        }

        if abicoder.contains(&"v2") {
            if let Some(lower) = lower {
                if lower >= 0.8 {
                    self.report.borrow_mut().add_entry(
                        source_unit_path,
                        source_line,
                        "Unnecessary specification of `pragma abicoder v2`, which is enabled in Solidity v0.8.0 and above",
                    )
                }
            }
        }

        solidity.clear();
        abicoder.clear();
    }
}

impl AstVisitor for UnnecessaryPragmasVisitor {
    fn visit_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> {
        let mut solidity: Vec<&str> = vec![];
        let mut abicoder: Vec<&str> = vec![];

        for pragma_directive in context.current_source_unit.pragma_directives() {
            match pragma_directive.literals.first().map(String::as_str) {
                Some("solidity") => {
                    if !solidity.is_empty() {
                        self.check_pragma_directives(
                            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                            Some(context.current_source_unit.source_line(pragma_directive.src.as_str())?),
                            &mut solidity,
                            &mut abicoder
                        );
                    }

                    solidity.extend(pragma_directive.literals.iter().skip(1).map(String::as_str));
                }

                Some("abicoder") => {
                    if !abicoder.is_empty() {
                        self.check_pragma_directives(
                            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                            Some(context.current_source_unit.source_line(pragma_directive.src.as_str())?),
                            &mut solidity,
                            &mut abicoder
                        );
                    }

                    abicoder.extend(pragma_directive.literals.iter().skip(1).map(String::as_str));
                }

                _ => {}
            }
        }

        self.check_pragma_directives(
            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
            None,
            &mut solidity,
            &mut abicoder
        );

        Ok(())
    }
}
