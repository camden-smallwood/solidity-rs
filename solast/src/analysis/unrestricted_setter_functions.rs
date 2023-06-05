use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct UnrestrictedSetterFunctionsVisitor {
    report: Rc<RefCell<Report>>,
}

impl UnrestrictedSetterFunctionsVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn print_message(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} is an unprotected setter function",
                contract_definition.definition_node_location(definition_node),
            ),
        );
    }
}

impl AstVisitor for UnrestrictedSetterFunctionsVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if let FunctionKind::Constructor = context.function_definition.kind {
            return Ok(())
        }

        if let Visibility::Private | Visibility::Internal = context.function_definition.visibility {
            return Ok(())
        }

        if let StateMutability::Pure | StateMutability::View = context.function_definition.state_mutability {
            return Ok(())
        }

        if !context.function_definition.modifiers.is_empty() {
            //
            // TODO: check for onlyOwner-like modifiers?
            //

            return Ok(())
        }

        match context.function_definition.body.as_ref() {
            Some(block) if !block.statements.is_empty() => {}
            _ => return Ok(())
        }

        for statement in context.function_definition.body.as_ref().unwrap().statements.iter() {
            match statement {
                Statement::ExpressionStatement(ExpressionStatement {
                    expression: Expression::Assignment(_),
                }) => continue,

                _ => return Ok(())
            }
        }

        self.print_message(
            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
            context.contract_definition,
            context.definition_node,
            context.current_source_unit.source_line(context.function_definition.src.as_str())?,
        );

        Ok(())
    }
}
