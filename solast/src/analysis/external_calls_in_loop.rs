use super::{AstVisitor, FunctionDefinitionContext};
use solidity::ast::{NodeID, SourceUnit};
use std::io;

pub struct ExternalCallsInLoopVisitor<'a> {
    source_units: &'a [SourceUnit],
    loop_ids: Vec<NodeID>,
    makes_external_call: bool,
}

impl<'a> ExternalCallsInLoopVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self {
            source_units,
            loop_ids: vec![],
            makes_external_call: false,
        }
    }
}

impl AstVisitor for ExternalCallsInLoopVisitor<'_> {
    fn visit_function_definition<'a>(&mut self, _context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        self.loop_ids.clear();
        self.makes_external_call = false;

        Ok(())
    }

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if self.makes_external_call {
            println!(
                "\t{} {} {} makes an external call inside a loop",

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
        definition_node: &solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&solidity::ast::Block>,
        _statement: Option<&solidity::ast::Statement>,
        identifier: &solidity::ast::Identifier,
    ) -> io::Result<()> {
        match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(_) => {}
            _ => return Ok(())
        }

        if self.loop_ids.is_empty() || self.makes_external_call {
            return Ok(());
        }

        for source_unit in self.source_units.iter() {
            let function_definition =
                match source_unit.function_definition(identifier.referenced_declaration) {
                    Some(function_definition) => function_definition,
                    None => continue,
                };

            if let solidity::ast::Visibility::External = function_definition.visibility {
                self.makes_external_call = true;
                break;
            }
        }

        Ok(())
    }

    fn visit_member_access<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        member_access: &'a solidity::ast::MemberAccess,
    ) -> io::Result<()> {
        match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(_) => {}
            _ => return Ok(())
        }

        if self.loop_ids.is_empty() || self.makes_external_call {
            return Ok(());
        }

        if let Some(referenced_declaration) = member_access.referenced_declaration {
            for source_unit in self.source_units.iter() {
                if let Some(function_definition) =
                    source_unit.function_definition(referenced_declaration)
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
}
