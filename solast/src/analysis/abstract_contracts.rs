use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, rc::Rc};

pub struct AbstractContractsVisitor {
    report: Rc<RefCell<Report>>,
}

impl AbstractContractsVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }
}

impl AstVisitor for AbstractContractsVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> std::io::Result<()> {
        //
        // Only check function definitions associated with constructors
        //

        if context.function_definition.kind != FunctionKind::Constructor {
            return Ok(())
        }

        //
        // Only check function definitions with internal visibility
        //

        if context.function_definition.visibility != Visibility::Internal {
            return Ok(())
        }

        //
        // If the constructor is marked internal and the contract is not abstract, print a message
        //

        if let None | Some(false) = context.contract_definition.is_abstract {
            self.report.borrow_mut().add_entry(
                context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                Some(context.current_source_unit.source_line(context.function_definition.src.as_str())?),
                format!(
                    "{} is marked {} instead of marking `{}` as abstract",
                    context.contract_definition.definition_node_location(context.definition_node),
                    context.function_definition.visibility,
                    context.contract_definition.name,
                ),
            );
        }

        Ok(())
    }
}
