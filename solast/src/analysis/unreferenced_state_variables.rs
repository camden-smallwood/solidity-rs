use super::{AstVisitor, VariableDeclarationContext};
use solidity::ast::{
    Block, ContractDefinition, ContractDefinitionNode, FunctionKind, Identifier, MemberAccess,
    NodeID, SourceUnit, Statement,
};
use std::{collections::HashMap, io};

pub struct ContractInfo {
    variable_info: HashMap<NodeID, bool>,
}

pub struct UnusedStateVariablesVisitor {
    contract_info: HashMap<NodeID, ContractInfo>,
}

impl Default for UnusedStateVariablesVisitor {
    fn default() -> Self {
        Self {
            contract_info: HashMap::new(),
        }
    }
}

impl AstVisitor for UnusedStateVariablesVisitor {
    fn visit_contract_definition<'a>(&mut self, context: &mut super::ContractDefinitionContext<'a>) -> io::Result<()> {
        if !self.contract_info.contains_key(&context.contract_definition.id) {
            self.contract_info.insert(context.contract_definition.id, ContractInfo {
                variable_info: HashMap::new(),
            });
        }

        Ok(())
    }
    
    fn leave_contract_definition<'a>(&mut self, context: &mut super::ContractDefinitionContext<'a>) -> io::Result<()> {
        if let Some(contract_info) = self.contract_info.get(&context.contract_definition.id) {
            for (&id, &referenced) in contract_info.variable_info.iter() {
                if let Some(variable_declaration) = context.contract_definition.variable_declaration(id) {
                    if let Some(solidity::ast::Mutability::Constant) = variable_declaration.mutability.as_ref() {
                        continue;
                    }

                    if !referenced {
                        println!(
                            "\tThe {} `{}.{}` {} state variable is never referenced",
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
            let contract_info = self.contract_info.get_mut(&context.contract_definition.id).unwrap();

            if !contract_info.variable_info.contains_key(&variable_declaration.id) {
                contract_info.variable_info.insert(variable_declaration.id, false);
            }
        }

        Ok(())
    }

    fn visit_identifier(
        &mut self,
        _source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        _blocks: &mut Vec<&Block>,
        _statement: Option<&Statement>,
        identifier: &Identifier,
    ) -> io::Result<()> {
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) if function_definition.kind != FunctionKind::Constructor => {}
            ContractDefinitionNode::ModifierDefinition(_) => {}
            _ => return Ok(())
        }

        let contract_info = self.contract_info.get_mut(&contract_definition.id).unwrap();

        if let Some(variable_info) = contract_info.variable_info.get_mut(&identifier.referenced_declaration) {
            *variable_info = true;
        }

        Ok(())
    }

    fn visit_member_access<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        member_access: &'a MemberAccess,
    ) -> io::Result<()> {
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) if function_definition.kind != FunctionKind::Constructor => {}
            ContractDefinitionNode::ModifierDefinition(_) => {}
            _ => return Ok(())
        }

        let contract_info = self.contract_info.get_mut(&contract_definition.id).unwrap();

        if let Some(referenced_declaration) = member_access.referenced_declaration {
            if let Some(variable_info) = contract_info.variable_info.get_mut(&referenced_declaration) {
                *variable_info = true;
            }
        }

        Ok(())
    }
}
