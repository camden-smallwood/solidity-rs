use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::collections::HashMap;

struct SourceUnitInfo {
    imported_paths: HashMap<String, bool>,
}

#[derive(Default)]
pub struct RedundantImportsVisitor {
    source_unit_info: HashMap<NodeID, SourceUnitInfo>,
}

impl AstVisitor for RedundantImportsVisitor {
    fn visit_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> std::io::Result<()> {
        if !self.source_unit_info.contains_key(&context.current_source_unit.id) {
            self.source_unit_info.insert(context.current_source_unit.id, SourceUnitInfo {
                imported_paths: HashMap::new(),
            });
        }

        Ok(())
    }
    
    fn visit_import_directive<'a>(&mut self, context: &mut ImportDirectiveContext<'a>) -> std::io::Result<()> {
        let source_unit_info = self.source_unit_info.get_mut(&context.current_source_unit.id).unwrap();

        match source_unit_info.imported_paths.get_mut(&context.import_directive.file) {
            Some(reported) => if !*reported {
                println!(
                    "\tL{}: Redundant import specified: `{}`",
                    context.current_source_unit.source_line(context.import_directive.src.as_str())?,
                    context.import_directive.file
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
