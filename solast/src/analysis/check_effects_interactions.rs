use super::{AstVisitor, FunctionDefinitionContext};
use solidity::ast::{NodeID, SourceUnit};
use std::{collections::HashMap, io};

pub struct CheckEffectsInteractionsVisitor<'a> {
    pub source_units: &'a [SourceUnit],
    pub makes_external_call: bool,
    pub makes_post_external_call_assignment: bool,
    pub bindings: HashMap<NodeID, Vec<NodeID>>,
}

impl<'a> CheckEffectsInteractionsVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self {
            source_units,
            makes_external_call: false,
            makes_post_external_call_assignment: false,
            bindings: HashMap::new(),
        }
    }
}

impl AstVisitor for CheckEffectsInteractionsVisitor<'_> {
    fn visit_function_definition<'a>(&mut self, _context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        self.makes_external_call = false;
        self.makes_post_external_call_assignment = false;

        Ok(())
    }

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if let solidity::ast::FunctionKind::Constructor = context.function_definition.kind {
            return Ok(());
        }

        if self.makes_external_call && self.makes_post_external_call_assignment {
            println!(
                "\t{} {} {} ignores the Check-Effects-Interactions pattern",

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

    fn visit_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        statement: &'a solidity::ast::Statement,
    ) -> io::Result<()> {
        if let solidity::ast::Statement::VariableDeclarationStatement(
            solidity::ast::VariableDeclarationStatement {
                declarations,
                initial_value: Some(expression),
                ..
            },
        ) = statement
        {
            let ids = contract_definition.get_assigned_state_variables(
                self.source_units,
                definition_node,
                expression,
            );

            for &id in ids.iter() {
                if contract_definition.hierarchy_contains_state_variable(self.source_units, id) {
                    let state_variable = {
                        let mut state_variable = None;

                        if let Some(contract_ids) = contract_definition.linearized_base_contracts.as_ref() {
                            for &contract_id in contract_ids.iter() {
                                for source_unit in self.source_units.iter() {
                                    if let Some(contract_definition) = source_unit.contract_definition(contract_id) {
                                        if let Some(variable_declaration) = contract_definition.variable_declaration(id) {
                                            state_variable = Some(variable_declaration);
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(variable_declaration) = contract_definition.variable_declaration(id) {
                                state_variable = Some(variable_declaration);
                            }
                        }

                        state_variable.unwrap()
                    };

                    if declarations.len() > 1 {
                        println!(
                            "\tWARNING: tuple or multiple assignments not handled: {:?} {:#?}",
                            ids, declarations
                        );
                    } else {
                        let declaration = match declarations.first().unwrap().as_ref() {
                            Some(declaration) => declaration,
                            None => return Ok(()),
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

    fn visit_identifier(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&solidity::ast::Block>,
        _statement: Option<&solidity::ast::Statement>,
        identifier: &solidity::ast::Identifier,
    ) -> io::Result<()> {
        if self.makes_external_call {
            return Ok(());
        }

        for source_unit in self.source_units.iter() {
            if let Some(function_definition) =
                source_unit.function_definition(identifier.referenced_declaration)
            {
                if let solidity::ast::Visibility::External = function_definition.visibility {
                    self.makes_external_call = true;
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn visit_member_access<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        member_access: &'a solidity::ast::MemberAccess,
    ) -> io::Result<()> {
        if self.makes_external_call {
            return Ok(());
        }

        if let Some(referenced_declaration) = member_access.referenced_declaration {
            for source_unit in self.source_units.iter() {
                if let Some(function_definition) = source_unit.function_definition(referenced_declaration)
                {
                    if let solidity::ast::Visibility::External = function_definition.visibility {
                        self.makes_external_call = true;
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn visit_assignment<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        assignment: &'a solidity::ast::Assignment,
    ) -> io::Result<()> {
        let function_definition = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(())
        };

        if !self.makes_external_call {
            return Ok(());
        }

        if self.makes_post_external_call_assignment {
            return Ok(());
        }

        if function_definition
            .modifiers
            .iter()
            .find(|m| m.modifier_name.name == "nonReentrant")
            .is_some()
        {
            return Ok(());
        }

        let ids = contract_definition.get_assigned_state_variables(
            self.source_units,
            definition_node,
            assignment.left_hand_side.as_ref(),
        );

        if !ids.is_empty() {
            self.makes_post_external_call_assignment = true;
        }

        match assignment.left_hand_side.as_ref() {
            solidity::ast::Expression::Identifier(_) => {
                // TODO: check if local variable is no longer bound to state variable
            }

            solidity::ast::Expression::IndexAccess(_)
            | solidity::ast::Expression::IndexRangeAccess(_)
            | solidity::ast::Expression::MemberAccess(_) => {
                match assignment.left_hand_side.root_expression() {
                    Some(solidity::ast::Expression::Identifier(solidity::ast::Identifier {
                        referenced_declaration,
                        ..
                    })) => {
                        for (_state_variable_id, local_variable_ids) in self.bindings.iter() {
                            if local_variable_ids.contains(referenced_declaration) {
                                self.makes_post_external_call_assignment = true;
                                return Ok(());
                            }
                        }
                    }

                    _ => {}
                }
            }

            solidity::ast::Expression::TupleExpression(tuple_expression) => {
                for component in tuple_expression.components.iter() {
                    if let Some(component) = component {
                        match component {
                            solidity::ast::Expression::Identifier(_) => {
                                // TODO: check if local variable is no longer bound to state variable
                            }

                            solidity::ast::Expression::IndexAccess(_)
                            | solidity::ast::Expression::IndexRangeAccess(_)
                            | solidity::ast::Expression::MemberAccess(_) => {
                                match component.root_expression() {
                                    Some(solidity::ast::Expression::Identifier(
                                        solidity::ast::Identifier {
                                            referenced_declaration,
                                            ..
                                        },
                                    )) => {
                                        for (_state_variable_id, local_variable_ids) in
                                            self.bindings.iter()
                                        {
                                            if local_variable_ids.contains(referenced_declaration) {
                                                self.makes_post_external_call_assignment = true;
                                                return Ok(());
                                            }
                                        }
                                    }

                                    _ => {}
                                }
                            }

                            expression => {
                                println!(
                                    "\tWARNING: unhandled assignment in tuple {:#?}",
                                    expression
                                );
                            }
                        }
                    }
                }
            }

            expression => {
                println!("\tWARNING: unhandled assignment {:#?}", expression);
            }
        }

        Ok(())
    }
}
