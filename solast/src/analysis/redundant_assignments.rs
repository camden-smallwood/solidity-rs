use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::io;

//
// TODO:
//   determine if something is assigned to, then re-assigned to without being referenced
//

pub struct RedundantAssignmentsVisitor;

impl RedundantAssignmentsVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        assignment: &Assignment
    ) {
        println!(
            "\t{} contains a redundant assignment: `{}`",
            contract_definition.definition_node_location(source_line, definition_node),
            assignment,
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
                    self.print_message(
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
