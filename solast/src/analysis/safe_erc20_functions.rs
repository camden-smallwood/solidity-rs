use super::AstVisitor;
use solidity::ast::{NodeID, SourceUnit};
use std::{collections::HashMap, io};

struct FunctionInfo {
    transfer: bool,
    transfer_from: bool,
    approve: bool,
}

pub struct SafeERC20FunctionsVisitor<'a> {
    pub source_units: &'a [SourceUnit],
    functions: HashMap<NodeID, FunctionInfo>,
}

impl<'a> SafeERC20FunctionsVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self {
            source_units,
            functions: HashMap::new(),
        }
    }
}

impl AstVisitor for SafeERC20FunctionsVisitor<'_> {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if !self.functions.contains_key(&function_definition.id) {
            self.functions.insert(
                function_definition.id,
                FunctionInfo {
                    transfer: false,
                    transfer_from: false,
                    approve: false,
                },
            );
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
        let function_info = self.functions.get(&function_definition.id).unwrap();

        if function_info.transfer {
            println!(
                "\t{} {} {} uses ERC20.transfer instead of SafeERC20.safeTransfer",
                format!("{:?}", function_definition.visibility),
                if function_definition.name.is_empty() {
                    format!("{}", contract_definition.name)
                } else {
                    format!("{}.{}", contract_definition.name, function_definition.name)
                },
                format!("{:?}", function_definition.kind).to_lowercase()
            );
        }

        if function_info.transfer_from {
            println!(
                "\t{} {} {} uses ERC20.transferFrom instead of SafeERC20.safeTransferFrom",
                format!("{:?}", function_definition.visibility),
                if function_definition.name.is_empty() {
                    format!("{}", contract_definition.name)
                } else {
                    format!("{}.{}", contract_definition.name, function_definition.name)
                },
                format!("{:?}", function_definition.kind).to_lowercase()
            );
        }

        if function_info.approve {
            println!(
                "\t{} {} {} uses ERC20.approve instead of SafeERC20.safeApprove",
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

    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
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

        if contract_definition.name == "SafeERC20" {
            return Ok(());
        }

        let function_info = match self.functions.get_mut(&definition_id) {
            Some(function_info) => function_info,
            None => return Ok(())
        };

        for referenced_declaration in function_call.expression.referenced_declarations() {
            for source_unit in self.source_units.iter() {
                if let Some((called_contract_definition, called_function_definition)) =
                    source_unit.function_and_contract_definition(referenced_declaration)
                {
                    if let "erc20" | "ierc20" = called_contract_definition
                        .name
                        .to_ascii_lowercase()
                        .as_str()
                    {
                        match called_function_definition.name.as_str() {
                            "transfer" => {
                                function_info.transfer = true;
                            }

                            "transferFrom" => {
                                function_info.transfer_from = true;
                            }

                            "approve" => {
                                function_info.approve = true;
                            }

                            _ => {}
                        }
                    }

                    break;
                }
            }
        }

        Ok(())
    }
}
