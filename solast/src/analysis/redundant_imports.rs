use crate::report::Report;
use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct SourceUnitInfo {
    imported_paths: HashMap<String, bool>,
}

pub struct RedundantImportsVisitor {
    report: Rc<RefCell<Report>>,
    source_unit_info: HashMap<NodeID, SourceUnitInfo>,
}

impl RedundantImportsVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self {
            report,
            source_unit_info: HashMap::new(),
        }
    }
}

impl AstVisitor for RedundantImportsVisitor {
    fn visit_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> std::io::Result<()> {
        self.source_unit_info.entry(context.current_source_unit.id).or_insert_with(|| SourceUnitInfo {
            imported_paths: HashMap::new(),
        });

        Ok(())
    }
    
    fn visit_import_directive<'a>(&mut self, context: &mut ImportDirectiveContext<'a>) -> std::io::Result<()> {
        let source_unit_info = self.source_unit_info.get_mut(&context.current_source_unit.id).unwrap();

        match source_unit_info.imported_paths.get_mut(&context.import_directive.file) {
            Some(reported) => if !*reported {
                self.report.borrow_mut().add_entry(
                    context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                    Some(context.current_source_unit.source_line(context.import_directive.src.as_str())?),
                    format!("Redundant import specified: `{}`", context.import_directive.file),
                );
                *reported = true;
            }

            None => {
                source_unit_info.imported_paths.insert(context.import_directive.file.clone(), false);
            }
        }

        Ok(())
    }
}
