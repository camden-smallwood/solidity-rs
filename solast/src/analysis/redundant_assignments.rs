use solidity::ast::{ContractDefinitionNode, Expression, FunctionKind, NodeID};

use super::AstVisitor;

pub struct RedundantAssignmentsVisitor;

//
// TODO:
//   determine if something is assigned to, then re-assigned to without being referenced
//

impl AstVisitor for RedundantAssignmentsVisitor {
    fn visit_assignment<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        assignment: &'a solidity::ast::Assignment,
    ) -> std::io::Result<()> {
        if let Expression::TupleExpression(tuple_expression) = assignment.left_hand_side.as_ref() {
            let mut tuple_component_ids: Vec<Vec<NodeID>> = vec![];

            for component in tuple_expression.components.iter() {
                let mut component_ids = vec![];

                if let Some(component) = component.as_ref() {
                    component_ids.extend(component.referenced_declarations());
                }

                if tuple_component_ids.iter().find(|&ids| ids.eq(&component_ids)).is_some() {
                    match definition_node {
                        ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                            "\tThe {} {} in the `{}` {} contains a redundant assignment: `{}`",

                            function_definition.visibility,

                            if let FunctionKind::Constructor = function_definition.kind {
                                format!("{}", "constructor")
                            } else {
                                format!("`{}` {}", function_definition.name, function_definition.kind)
                            },

                            contract_definition.name,
                            contract_definition.kind,

                            assignment
                        ),

                        ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                            "\tThe `{}` modifier in the `{}` {} contains a redundant assignment: `{}`",
                            
                            modifier_definition.name,

                            contract_definition.name,
                            contract_definition.kind,

                            assignment
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
