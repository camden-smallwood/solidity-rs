use super::AstVisitor;
use solidity::ast::{ContractDefinition, ContractDefinitionNode, FunctionDefinition, FunctionKind, SourceUnit, Visibility};
use std::io;

pub struct AbstractContractsVisitor;

impl AstVisitor for AbstractContractsVisitor {
    fn visit_function_definition(
        &mut self,
        _source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        _definition_node: &ContractDefinitionNode,
        function_definition: &FunctionDefinition,
    ) -> io::Result<()> {
        if function_definition.kind != FunctionKind::Constructor {
            return Ok(())
        }

        if function_definition.visibility != Visibility::Internal {
            return Ok(())
        }

        match contract_definition.is_abstract {
            None | Some(false) => {
                println!(
                    "\t{:?} {} {} is marked {} instead of marking {} as abstract.",
                    contract_definition.kind,
                    contract_definition.name,
                    function_definition.kind,
                    function_definition.visibility,
                    contract_definition.name,
                );
            }

            Some(true) => {
                if function_definition.visibility == Visibility::Internal {
                    println!(
                        "\t{:?} {} {} is marked {} when {} is already marked as abstract.",
                        contract_definition.kind,
                        contract_definition.name,
                        function_definition.kind,
                        function_definition.visibility,
                        contract_definition.name,
                    );
                }
            }
        }

        Ok(())
    }
}
