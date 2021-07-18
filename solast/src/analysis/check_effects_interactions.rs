use super::{AstVisitor, FunctionDefinitionContext, IdentifierContext, StatementContext};
use solidity::ast::*;
use std::{collections::HashMap, io};

pub struct CheckEffectsInteractionsVisitor {
    makes_external_call: bool,
    makes_post_external_call_assignment: bool,
    bindings: HashMap<NodeID, Vec<NodeID>>,
}

impl Default for CheckEffectsInteractionsVisitor {
    fn default() -> Self {
        Self {
            makes_external_call: false,
            makes_post_external_call_assignment: false,
            bindings: HashMap::new(),
        }
    }
}

impl AstVisitor for CheckEffectsInteractionsVisitor {
    fn visit_function_definition<'a>(&mut self, _context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        self.makes_external_call = false;
        self.makes_post_external_call_assignment = false;

        Ok(())
    }

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if let FunctionKind::Constructor = context.function_definition.kind {
            return Ok(())
        }

        if self.makes_external_call && self.makes_post_external_call_assignment {
            println!(
                "\tL{}: {} {} {} ignores the Check-Effects-Interactions pattern",

                context.current_source_unit.source_line(context.function_definition.src.as_str()).unwrap(),

                format!("{:?}", context.function_definition.visibility),

                if context.function_definition.name.is_empty() {
                    format!("{}", context.contract_definition.name)
                } else {
                    format!("{}.{}", context.contract_definition.name, context.function_definition.name)
                },
                
                context.function_definition.kind
            );
        }

        Ok(())
    }

    fn visit_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> io::Result<()> {
        if let Statement::VariableDeclarationStatement(VariableDeclarationStatement {
            declarations,
            initial_value: Some(expression),
            ..
        }) = context.statement {
            let ids = context.contract_definition.get_assigned_state_variables(
                context.source_units,
                context.definition_node,
                expression
            );

            for &id in ids.iter() {
                if context.contract_definition.hierarchy_contains_state_variable(context.source_units, id) {
                    let state_variable = {
                        let mut state_variable = None;

                        if let Some(contract_ids) = context.contract_definition.linearized_base_contracts.as_ref() {
                            for &contract_id in contract_ids.iter() {
                                for source_unit in context.source_units.iter() {
                                    if let Some(contract_definition) = source_unit.contract_definition(contract_id) {
                                        if let Some(variable_declaration) = contract_definition.variable_declaration(id) {
                                            state_variable = Some(variable_declaration);
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(variable_declaration) = context.contract_definition.variable_declaration(id) {
                                state_variable = Some(variable_declaration);
                            }
                        }

                        state_variable.unwrap()
                    };

                    if declarations.len() > 1 {
                        //
                        // TODO: handle tuple or multiple assignments (is this actually necessary?)
                        //
                        
                        println!(
                            "\tWARNING: tuple or multiple assignments not handled: {:?} {:#?}",
                            ids, declarations
                        );
                    } else {
                        let declaration = match declarations.first().unwrap().as_ref() {
                            Some(declaration) => declaration,
                            None => return Ok(())
                        };

                        if !self.bindings.contains_key(&state_variable.id) {
                            self.bindings.insert(state_variable.id, vec![]);
                        }

                        let bindings = self.bindings.get_mut(&state_variable.id).unwrap();

                        if !bindings.contains(&declaration.id) {
                            bindings.push(declaration.id);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn visit_identifier<'a, 'b>(&mut self, context: &mut IdentifierContext<'a, 'b>) -> io::Result<()> {
        if self.makes_external_call {
            return Ok(())
        }

        for source_unit in context.source_units.iter() {
            if let Some(function_definition) = source_unit.function_definition(context.identifier.referenced_declaration) {
                if let Visibility::External = function_definition.visibility {
                    self.makes_external_call = true;
                    return Ok(())
                }
            }
        }

        Ok(())
    }

    fn visit_member_access<'a, 'b>(&mut self, context: &mut super::MemberAccessContext<'a, 'b>) -> io::Result<()> {
        if self.makes_external_call {
            return Ok(())
        }

        if let Some(referenced_declaration) = context.member_access.referenced_declaration {
            for source_unit in context.source_units.iter() {
                if let Some(function_definition) = source_unit.function_definition(referenced_declaration) {
                    if let Visibility::External = function_definition.visibility {
                        self.makes_external_call = true;
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn visit_assignment<'a, 'b>(&mut self, context: &mut super::AssignmentContext<'a, 'b>) -> io::Result<()> {
        let function_definition = match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(())
        };

        if !self.makes_external_call {
            return Ok(())
        }

        if self.makes_post_external_call_assignment {
            return Ok(())
        }

        // TODO: remove this ugly hack and check modifiers correctly
        if function_definition.modifiers.iter().find(|m| m.modifier_name.name == "nonReentrant").is_some() {
            return Ok(())
        }

        let ids = context.contract_definition.get_assigned_state_variables(
            context.source_units,
            context.definition_node,
            context.assignment.left_hand_side.as_ref(),
        );

        if !ids.is_empty() {
            self.makes_post_external_call_assignment = true;
        }

        // TODO: refactor this into a function, there's likely some edge cases missed
        match context.assignment.left_hand_side.as_ref() {
            Expression::Identifier(_) => {
                // TODO: check if local variable is no longer bound to state variable
            }

            Expression::IndexAccess(_)
            | Expression::IndexRangeAccess(_)
            | Expression::MemberAccess(_) => {
                if let Some(Expression::Identifier(Identifier {
                    referenced_declaration,
                    ..
                })) = context.assignment.left_hand_side.root_expression() {
                    for (_state_variable_id, local_variable_ids) in self.bindings.iter() {
                        if local_variable_ids.contains(referenced_declaration) {
                            self.makes_post_external_call_assignment = true;
                            return Ok(())
                        }
                    }
                }
            }

            Expression::TupleExpression(tuple_expression) => {
                for component in tuple_expression.components.iter() {
                    if let Some(component) = component {
                        match component {
                            Expression::Identifier(_) => {
                                // TODO: check if local variable is no longer bound to state variable
                            }

                            Expression::IndexAccess(_)
                            | Expression::IndexRangeAccess(_)
                            | Expression::MemberAccess(_) => {
                                if let Some(Expression::Identifier(Identifier {
                                    referenced_declaration,
                                    ..
                                })) = component.root_expression() {
                                    for (_state_variable_id, local_variable_ids) in self.bindings.iter() {
                                        if local_variable_ids.contains(referenced_declaration) {
                                            self.makes_post_external_call_assignment = true;
                                            return Ok(())
                                        }
                                    }
                                }
                            }

                            expression => println!(
                                "\tWARNING: unhandled assignment in tuple {:#?}",
                                expression
                            )
                        }
                    }
                }
            }

            expression => println!("\tWARNING: unhandled assignment {:#?}", expression)
        }

        Ok(())
    }
}
