use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::{collections::HashMap, io};

pub struct ContractInfo {
    variable_info: HashMap<NodeID, bool>,
}

#[derive(Default)]
pub struct UnusedStateVariablesVisitor {
    contract_info: HashMap<NodeID, ContractInfo>,
}

impl AstVisitor for UnusedStateVariablesVisitor {
    fn visit_contract_definition<'a>(&mut self, context: &mut ContractDefinitionContext<'a>) -> io::Result<()> {
        self.contract_info.entry(context.contract_definition.id).or_insert_with(|| ContractInfo {
            variable_info: HashMap::new(),
        });

        Ok(())
    }
    
    fn leave_contract_definition<'a>(&mut self, context: &mut ContractDefinitionContext<'a>) -> io::Result<()> {
        if let Some(contract_info) = self.contract_info.get(&context.contract_definition.id) {
            for (&id, &referenced) in contract_info.variable_info.iter() {
                if let Some(variable_declaration) = context.contract_definition.variable_declaration(id) {
                    if let Some(solidity::ast::Mutability::Constant) = variable_declaration.mutability.as_ref() {
                        continue;
                    }

                    if !referenced {
                        println!(
                            "\tL{}: The {} `{}.{}` {} state variable is never referenced",

                            context.current_source_unit.source_line(variable_declaration.src.as_str())?,

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
        let contract_definition = match context.contract_definition.as_ref() {
            Some(contract_definition) => contract_definition,
            None => return Ok(())
        };

        let definition_node = match context.definition_node.as_ref() {
            Some(definition_node) => definition_node,
            None => return Ok(())
        };

        if let ContractDefinitionNode::VariableDeclaration(variable_declaration) = definition_node {
            let contract_info = self.contract_info.get_mut(&contract_definition.id).unwrap();

            contract_info.variable_info.entry(variable_declaration.id).or_insert_with(|| false);
        }

        Ok(())
    }

    fn visit_identifier<'a, 'b>(&mut self, context: &mut IdentifierContext<'a, 'b>) -> io::Result<()> {
        match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) if function_definition.kind != FunctionKind::Constructor => {}
            ContractDefinitionNode::ModifierDefinition(_) => {}
            _ => return Ok(())
        }

        let contract_info = self.contract_info.get_mut(&context.contract_definition.id).unwrap();

        if let Some(variable_info) = contract_info.variable_info.get_mut(&context.identifier.referenced_declaration) {
            *variable_info = true;
        }

        Ok(())
    }

    fn visit_member_access<'a, 'b>(&mut self, context: &mut MemberAccessContext<'a, 'b>) -> io::Result<()> {
        match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) if function_definition.kind != FunctionKind::Constructor => {}
            ContractDefinitionNode::ModifierDefinition(_) => {}
            _ => return Ok(())
        }

        let contract_info = self.contract_info.get_mut(&context.contract_definition.id).unwrap();

        if let Some(referenced_declaration) = context.member_access.referenced_declaration {
            if let Some(variable_info) = contract_info.variable_info.get_mut(&referenced_declaration) {
                *variable_info = true;
            }
        }

        Ok(())
    }
}
