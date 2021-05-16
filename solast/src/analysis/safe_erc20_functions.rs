use super::AstVisitor;
use crate::truffle;
use solidity::ast::NodeID;
use std::{collections::HashMap, io};

struct FunctionInfo {
    transfer: bool,
    transfer_from: bool,
    approve: bool,
}

pub struct SafeERC20FunctionsVisitor<'a> {
    pub files: &'a [truffle::File],
    functions: HashMap<NodeID, FunctionInfo>,
}

impl<'a> SafeERC20FunctionsVisitor<'a> {
    pub fn new(files: &'a [truffle::File]) -> Self {
        Self {
            files,
            functions: HashMap::new(),
        }
    }
}

impl AstVisitor for SafeERC20FunctionsVisitor<'_> {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
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
        function_definition: Option<&'a solidity::ast::FunctionDefinition>,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        function_call: &'a solidity::ast::FunctionCall,
    ) -> io::Result<()> {
        let function_definition = match function_definition {
            Some(function_definition) => function_definition,
            None => return Ok(()),
        };

        if contract_definition.name == "SafeERC20" {
            return Ok(());
        }

        let function_info = self.functions.get_mut(&function_definition.id).unwrap();

        for referenced_declaration in function_call.expression.referenced_declarations() {
            for file in self.files.iter() {
                if let Some((called_contract_definition, called_function_definition)) =
                    file.function_and_contract_definition(referenced_declaration)
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
