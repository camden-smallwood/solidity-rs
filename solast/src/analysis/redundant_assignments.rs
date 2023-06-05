use crate::report::Report;
use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

//
// TODO:
//   determine if something is assigned to, then re-assigned to without being referenced
//

pub struct RedundantAssignmentsVisitor {
    report: Rc<RefCell<Report>>,
}

impl RedundantAssignmentsVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }
    
    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        assignment: &Assignment
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} contains a redundant assignment: `{}`",
                contract_definition.definition_node_location(definition_node),
                assignment,
            ),
        );
    }
}

impl AstVisitor for RedundantAssignmentsVisitor {
    fn visit_assignment<'a, 'b>(&mut self, context: &mut AssignmentContext<'a, 'b>) -> io::Result<()> {
        if let Expression::TupleExpression(tuple_expression) = context.assignment.left_hand_side.as_ref() {
            let mut tuple_component_ids: Vec<Vec<NodeID>> = vec![];

            for component in tuple_expression.components.iter() {
                let mut component_ids = vec![];

                if let Some(component) = component.as_ref() {
                    component_ids.extend(component.referenced_declarations());
                }

                if !component_ids.is_empty() && tuple_component_ids.iter().any(|ids| ids.eq(&component_ids)) {
                    self.add_report_entry(
                        context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(context.assignment.src.as_str())?,
                        context.assignment
                    );
                }

                tuple_component_ids.push(component_ids);
            }
        }

        Ok(())
    }
}
