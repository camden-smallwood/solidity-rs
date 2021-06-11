use super::AstVisitor;
use solidity::ast::{Block, ContractDefinition, ContractDefinitionNode, FunctionKind, SourceUnit, Statement};
use std::io;
use yul::InlineAssembly;

pub struct InlineAssemblyVisitor;

impl AstVisitor for InlineAssemblyVisitor {
    fn visit_inline_assembly<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: &'a Statement,
        _inline_assembly: &'a InlineAssembly,
    ) -> io::Result<()> {
        let function_definition = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(())
        };

        println!(
            "\t{} {} {} contains inline assembly usage",
            format!("{:?}", function_definition.visibility),
            if let FunctionKind::Constructor = function_definition.kind {
                format!("{}", contract_definition.name)
            } else {
                format!("{}.{}", contract_definition.name, function_definition.name)
            },
            function_definition.kind,
        );

        Ok(())
    }
}
