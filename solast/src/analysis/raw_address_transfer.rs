use super::AstVisitor;
use solidity::ast::{NodeID, SourceUnit};
use std::{collections::HashMap, io};

pub struct RawAddressTransferVisitor<'a> {
    pub source_units: &'a [SourceUnit],
    functions_transfer: HashMap<NodeID, usize>,
}

impl<'a> RawAddressTransferVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self {
            source_units,
            functions_transfer: HashMap::new(),
        }
    }
}

impl AstVisitor for RawAddressTransferVisitor<'_> {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if !self
            .functions_transfer
            .contains_key(&function_definition.id)
        {
            self.functions_transfer.insert(function_definition.id, 0);
        }

        Ok(())
    }

    fn leave_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if let Some(&transfer_count) = self.functions_transfer.get(&function_definition.id) {
            if transfer_count > 0 {
                println!(
                    "\t{} {} {} performs {}",
                    format!("{:?}", function_definition.visibility),
                    if function_definition.name.is_empty() {
                        format!("{}", contract_definition.name)
                    } else {
                        format!("{}.{}", contract_definition.name, function_definition.name)
                    },
                    format!("{:?}", function_definition.kind).to_lowercase(),
                    if transfer_count == 1 {
                        "a raw address transfer"
                    } else {
                        "raw address transfers"
                    }
                );
            }
        }

        Ok(())
    }

    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        function_call: &'a solidity::ast::FunctionCall,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        if let solidity::ast::Expression::MemberAccess(member_access) =
            function_call.expression.as_ref()
        {
            if (member_access.referenced_declaration.is_none()
                || member_access
                    .referenced_declaration
                    .map(|id| id == 0)
                    .unwrap_or(false))
                && member_access.member_name == "transfer"
            {
                *self
                    .functions_transfer
                    .get_mut(&definition_id)
                    .unwrap() += 1;
            }
        }

        Ok(())
    }

    fn visit_function_call_options<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        function_call_options: &'a solidity::ast::FunctionCallOptions,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        if let solidity::ast::Expression::MemberAccess(member_access) =
            function_call_options.expression.as_ref()
        {
            if (member_access.referenced_declaration.is_none()
                || member_access
                    .referenced_declaration
                    .map(|id| id == 0)
                    .unwrap_or(false))
                && member_access.member_name == "transfer"
            {
                *self
                    .functions_transfer
                    .get_mut(&definition_id)
                    .unwrap() += 1;
            }
        }

        Ok(())
    }
}
