use super::{AstVisitor, CallGraph};
use crate::truffle;
use solidity::ast::NodeID;
use std::{collections::HashMap, io};

pub struct CheckEffectsInteractionsVisitor<'a, 'b> {
    pub files: &'a [truffle::File],
    pub call_graph: &'b CallGraph,
    pub makes_external_call: bool,
    pub makes_post_external_call_assignment: bool,
    pub bindings: HashMap<NodeID, Vec<NodeID>>,
}

impl<'a, 'b> CheckEffectsInteractionsVisitor<'a, 'b> {
    pub fn new(files: &'a [truffle::File], call_graph: &'b CallGraph) -> Self {
        Self {
            files,
            call_graph,
            makes_external_call: false,
            makes_post_external_call_assignment: false,
            bindings: HashMap::new(),
        }
    }
}

impl AstVisitor for CheckEffectsInteractionsVisitor<'_, '_> {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        _function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        self.makes_external_call = false;
        self.makes_post_external_call_assignment = false;

        Ok(())
    }

    fn leave_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if let solidity::ast::FunctionKind::Constructor = function_definition.kind {
            return Ok(());
        }

        if self.makes_external_call && self.makes_post_external_call_assignment {
            println!(
                "\t{} {} {} ignores the Check-Effects-Interactions pattern",
                format!("{:?}", function_definition.visibility),
                if function_definition.name.is_empty() {
                    format!("{}", contract_definition.name)
                } else {
                    format!("{}.{}", contract_definition.name, function_definition.name)
                },
                format!("{:?}", function_definition.kind).to_lowercase()
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
            let ids = self.call_graph.get_assigned_state_variables(
                self.files,
                contract_definition,
                definition_node,
                expression,
            )?;

            for &id in ids.iter() {
                if self.call_graph.hierarchy_contains_state_variable(self.files, contract_definition, id) {
                    let state_variable = {
                        let mut state_variable = None;

                        for &contract_id in contract_definition.linearized_base_contracts.iter() {
                            for file in self.files.iter() {
                                if let Some(contract_definition) =
                                    file.contract_definition(contract_id)
                                {
                                    if let Some(variable_declaration) =
                                        contract_definition.variable_declaration(id)
                                    {
                                        state_variable = Some(variable_declaration);
                                        break;
                                    }
                                }
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

        for file in self.files.iter() {
            if let Some(function_definition) =
                file.function_definition(identifier.referenced_declaration)
            {
                if let solidity::ast::Visibility::External = function_definition.visibility {
                    self.makes_external_call = true;
                    return Ok(());
                }
            }
        }

        if let Some(function_info) = self
            .call_graph
            .function_info(identifier.referenced_declaration)
        {
            if function_info.makes_external_call(self.files) {
                self.makes_external_call = true;
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
            for file in self.files.iter() {
                if let Some(function_definition) = file.function_definition(referenced_declaration)
                {
                    if let solidity::ast::Visibility::External = function_definition.visibility {
                        self.makes_external_call = true;
                        break;
                    }
                }
            }

            if let Some(function_info) = self.call_graph.function_info(referenced_declaration) {
                if function_info.makes_external_call(self.files) {
                    self.makes_external_call = true;
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

        let ids = self.call_graph.get_assigned_state_variables(
            self.files,
            contract_definition,
            definition_node,
            assignment.left_hand_side.as_ref(),
        )?;

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
