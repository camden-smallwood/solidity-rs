use super::{AstVisitor, VariableDeclarationContext};
use solidity::ast::{Assignment, Block, ContractDefinition, ContractDefinitionNode, Expression, FunctionCall, FunctionDefinition, FunctionKind, NodeID, SourceUnit, Statement, UnaryOperation};
use std::{collections::HashMap, io};

pub struct ContractInfo {
    variable_info: HashMap<NodeID, bool>,
}

pub struct StateVariableMutabilityVisitor<'a> {
    source_units: &'a [SourceUnit],
    contract_info: HashMap<NodeID, ContractInfo>,
}

impl<'a> StateVariableMutabilityVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self {
            source_units,
            contract_info: HashMap::new(),
        }
    }
}

impl AstVisitor for StateVariableMutabilityVisitor<'_> {
    fn leave_contract_definition<'a>(&mut self, context: &mut super::ContractDefinitionContext<'a>) -> io::Result<()> {
        if let Some(contract_info) = self.contract_info.get(&context.contract_definition.id) {
            for (&id, &assigned) in contract_info.variable_info.iter() {
                if let Some(variable_declaration) = context.contract_definition.variable_declaration(id) {
                    if let Some(solidity::ast::Mutability::Constant | solidity::ast::Mutability::Immutable) = variable_declaration.mutability.as_ref() {
                        continue;
                    }

                    if !assigned {
                        println!(
                            "\tThe {} `{}.{}` {} state variable can be declared `immutable`",
                            variable_declaration.visibility,
                            context.contract_definition.name,
                            variable_declaration.name,
                            variable_declaration.type_name.as_ref().unwrap(),
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn visit_variable_declaration<'a, 'b>(&mut self, context: &mut VariableDeclarationContext<'a, 'b>) -> io::Result<()> {
        if let ContractDefinitionNode::VariableDeclaration(variable_declaration) = context.definition_node {
            if !self.contract_info.contains_key(&context.contract_definition.id) {
                self.contract_info.insert(context.contract_definition.id, ContractInfo {
                    variable_info: HashMap::new(),
                });
            }

            let contract_info = match self.contract_info.get_mut(&context.contract_definition.id) {
                Some(contract_info) => contract_info,
                None => return Ok(())
            };

            if !contract_info.variable_info.contains_key(&variable_declaration.id) {
                contract_info.variable_info.insert(variable_declaration.id, false);
            }
        }

        Ok(())
    }

    fn visit_assignment<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        assignment: &'a Assignment,
    ) -> io::Result<()> {
        if let ContractDefinitionNode::FunctionDefinition(FunctionDefinition {
            kind: FunctionKind::Constructor,
            ..
        }) = definition_node {
            return Ok(())
        }
        
        let ids = contract_definition.get_assigned_state_variables(
            self.source_units,
            definition_node,
            assignment.left_hand_side.as_ref(),
        );

        for id in ids {
            let contract_info = match self.contract_info.get_mut(&contract_definition.id) {
                Some(contract_info) => contract_info,
                None => continue,
            };

            if contract_info.variable_info.contains_key(&id) {
                *contract_info.variable_info.get_mut(&id).unwrap() = true;
            }
        }

        Ok(())
    }

    fn visit_unary_operation<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        unary_operation: &'a UnaryOperation,
    ) -> io::Result<()> {
        if let ContractDefinitionNode::FunctionDefinition(FunctionDefinition {
            kind: FunctionKind::Constructor,
            ..
        }) = definition_node {
            return Ok(())
        }
        
        let ids = contract_definition.get_assigned_state_variables(
            self.source_units,
            definition_node,
            unary_operation.sub_expression.as_ref(),
        );

        for id in ids {
            let contract_info = match self.contract_info.get_mut(&contract_definition.id) {
                Some(contract_info) => contract_info,
                None => continue
            };

            if contract_info.variable_info.contains_key(&id) {
                *contract_info.variable_info.get_mut(&id).unwrap() = true;
            }
        }

        Ok(())
    }

    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        function_call: &'a FunctionCall,
    ) -> io::Result<()> {
        if let Expression::MemberAccess(member_access) = function_call.expression.as_ref() {
            if member_access.referenced_declaration.is_none() && (member_access.member_name == "push" || member_access.member_name == "pop") {
                if let ContractDefinitionNode::FunctionDefinition(FunctionDefinition {
                    kind: FunctionKind::Constructor,
                    ..
                }) = definition_node {
                    return Ok(())
                }
                
                let ids = contract_definition.get_assigned_state_variables(
                    self.source_units,
                    definition_node,
                    member_access.expression.as_ref(),
                );
                
                for id in ids {
                    let contract_info = match self.contract_info.get_mut(&contract_definition.id) {
                        Some(contract_info) => contract_info,
                        None => continue
                    };
        
                    if contract_info.variable_info.contains_key(&id) {
                        *contract_info.variable_info.get_mut(&id).unwrap() = true;
                    }
                }        
            }
        }
        
        Ok(())
    }
}
