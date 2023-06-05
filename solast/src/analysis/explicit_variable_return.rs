use crate::report::Report;
use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::{cell::RefCell, collections::HashSet, io, rc::Rc};

pub struct ExplicitVariableReturnVisitor{
    report: Rc<RefCell<Report>>,
    local_variable_ids: HashSet<NodeID>,
}

impl ExplicitVariableReturnVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self {
            report,
            local_variable_ids: HashSet::new(),
        }
    }
}

impl AstVisitor for ExplicitVariableReturnVisitor {
    fn visit_variable_declaration_statement<'a, 'b>(&mut self, context: &mut VariableDeclarationStatementContext<'a, 'b>) -> io::Result<()> {
        for declaration in context.variable_declaration_statement.declarations.iter().flatten() {
            if !self.local_variable_ids.contains(&declaration.id) {
                self.local_variable_ids.insert(declaration.id);
            }
        }

        Ok(())
    }

    fn visit_return<'a, 'b>(&mut self, context: &mut ReturnContext<'a, 'b>) -> io::Result<()> {
        let description = match context.return_statement.expression.as_ref() {
            Some(Expression::Identifier(identifier)) => {
                if self.local_variable_ids.contains(&identifier.referenced_declaration) {
                    Some("a local variable")
                } else {
                    None
                }
            }

            Some(Expression::TupleExpression(tuple_expression)) => {
                let mut all_local_variables = true;

                for component in tuple_expression.components.iter().flatten() {
                    if let Expression::Identifier(identifier) = component {
                        if !self.local_variable_ids.contains(&identifier.referenced_declaration) {
                            all_local_variables = false;
                            break;
                        }
                    } else {
                        all_local_variables = false;
                        break;
                    }
                }

                if all_local_variables {
                    Some("local variables")
                } else {
                    None
                }
            }

            _ => None,
        };

        if let Some(description) = description {
            self.report.borrow_mut().add_entry(
                context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                Some(context.current_source_unit.source_line(context.return_statement.src.as_str())?),
                format!(
                    "{} returns {} explicitly: `{}`",
                    context.contract_definition.definition_node_location(context.definition_node),
                    description,
                    context.return_statement
                ),
            );
        }

        Ok(())
    }
}
