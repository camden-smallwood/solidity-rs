use super::{AstVisitor, CallGraph};
use crate::truffle;
use solidity::ast::NodeID;
use std::io;

pub struct ExternalCallsInLoopVisitor<'a, 'b> {
    files: &'a [truffle::File],
    call_graph: &'b CallGraph,
    loop_ids: Vec<NodeID>,
    makes_external_call: bool,
}

impl<'a, 'b> ExternalCallsInLoopVisitor<'a, 'b> {
    pub fn new(files: &'a [truffle::File], call_graph: &'b CallGraph) -> Self {
        Self {
            files,
            call_graph,
            loop_ids: vec![],
            makes_external_call: false,
        }
    }
}

impl AstVisitor for ExternalCallsInLoopVisitor<'_, '_> {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        _function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        self.loop_ids.clear();
        self.makes_external_call = false;

        Ok(())
    }

    fn leave_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if self.makes_external_call {
            println!(
                "\t{} {} {} makes an external call inside a loop",
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

    fn visit_for_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        for_statement: &'a solidity::ast::ForStatement,
    ) -> io::Result<()> {
        self.loop_ids.push(for_statement.id);

        Ok(())
    }

    fn leave_for_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        for_statement: &'a solidity::ast::ForStatement,
    ) -> io::Result<()> {
        match self.loop_ids.pop() {
            Some(loop_id) => {
                if loop_id != for_statement.id {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "asdf"));
                }
            }

            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not currently in a loop",
                ))
            }
        }

        Ok(())
    }

    fn visit_while_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        while_statement: &'a solidity::ast::WhileStatement,
    ) -> io::Result<()> {
        self.loop_ids.push(while_statement.id);

        Ok(())
    }

    fn leave_while_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        while_statement: &'a solidity::ast::WhileStatement,
    ) -> io::Result<()> {
        match self.loop_ids.pop() {
            Some(loop_id) => {
                if loop_id != while_statement.id {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "asdf"));
                }
            }

            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not currently in a loop",
                ))
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
        if self.loop_ids.is_empty() || self.makes_external_call {
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
        if self.loop_ids.is_empty() || self.makes_external_call {
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
}
