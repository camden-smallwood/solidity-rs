use solidity::ast::*;

pub struct RedundantAssignmentsVisitor;

//
// TODO:
//   determine if something is assigned to, then re-assigned to without being referenced
//

impl AstVisitor for RedundantAssignmentsVisitor {
    fn visit_assignment<'a, 'b>(&mut self, context: &mut AssignmentContext<'a, 'b>) -> std::io::Result<()> {
        if let Expression::TupleExpression(tuple_expression) = context.assignment.left_hand_side.as_ref() {
            let mut tuple_component_ids: Vec<Vec<NodeID>> = vec![];

            for component in tuple_expression.components.iter() {
                let mut component_ids = vec![];

                if let Some(component) = component.as_ref() {
                    component_ids.extend(component.referenced_declarations());
                }

                if !component_ids.is_empty() && tuple_component_ids.iter().find(|&ids| ids.eq(&component_ids)).is_some() {
                    match context.definition_node {
                        ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                            "\tL{}: The {} {} in the `{}` {} contains a redundant assignment: `{}`",

                            context.current_source_unit.source_line(context.assignment.src.as_str()).unwrap(),

                            function_definition.visibility,

                            if function_definition.name.is_empty() {
                                format!("{}", function_definition.kind)
                            } else {
                                format!("`{}` {}", function_definition.name, function_definition.kind)
                            },

                            context.contract_definition.name,
                            context.contract_definition.kind,

                            context.assignment
                        ),

                        ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                            "\tL{}: The `{}` modifier in the `{}` {} contains a redundant assignment: `{}`",

                            context.current_source_unit.source_line(context.assignment.src.as_str()).unwrap(),

                            modifier_definition.name,

                            context.contract_definition.name,
                            context.contract_definition.kind,

                            context.assignment
                        ),

                        _ => return Ok(())
                    }
                }

                tuple_component_ids.push(component_ids);
            }
        }

        Ok(())
    }
}
