use crate::{analysis::AstVisitor, truffle};
use solidity::ast::{Block, ContractDefinition, ContractDefinitionNode, FunctionCall, SourceUnit, Statement};
use std::io;

pub struct UnpaidPayableFunctionsVisitor<'a> {
    files: &'a [truffle::File],
}

impl<'a> UnpaidPayableFunctionsVisitor<'a> {
    pub fn new(files: &'a [truffle::File]) -> Self {
        Self { files }
    }
}

impl AstVisitor for UnpaidPayableFunctionsVisitor<'_> {
    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        function_call: &'a FunctionCall,
    ) -> io::Result<()> {
        match function_call.expression.as_ref() {
            solidity::ast::Expression::Identifier(identifier) => {
                for file in self.files.iter() {
                    if let Some((called_contract_definition, called_function_definition)) = file.function_and_contract_definition(identifier.referenced_declaration) {
                        if called_function_definition.state_mutability == solidity::ast::StateMutability::Payable {
                            match definition_node {
                                ContractDefinitionNode::FunctionDefinition(function_definition) => {
                                    println!(
                                        "\t{} {} {} makes a call to the {} payable {} {} without paying",

                                        format!("{:?}", function_definition.visibility),

                                        if function_definition.name.is_empty() {
                                            format!("{}", contract_definition.name)
                                        } else {
                                            format!("{}.{}", contract_definition.name, function_definition.name)
                                        },

                                        format!("{:?}", function_definition.kind).to_lowercase(),

                                        format!("{:?}", called_function_definition.visibility).to_lowercase(),

                                        if called_function_definition.name.is_empty() {
                                            format!("{}", called_contract_definition.name)
                                        } else {
                                            format!("{}.{}", called_contract_definition.name, called_function_definition.name)
                                        },

                                        format!("{:?}", called_function_definition.kind).to_lowercase()
                                    );
                                }

                                ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                                    println!(
                                        "\t{} {} modifier makes a call to the {} payable {} {} without paying",

                                        format!("{:?}", modifier_definition.visibility),

                                        if modifier_definition.name.is_empty() {
                                            format!("{}", contract_definition.name)
                                        } else {
                                            format!("{}.{}", contract_definition.name, modifier_definition.name)
                                        },

                                        format!("{:?}", called_function_definition.visibility).to_lowercase(),

                                        if called_function_definition.name.is_empty() {
                                            format!("{}", called_contract_definition.name)
                                        } else {
                                            format!("{}.{}", called_contract_definition.name, called_function_definition.name)
                                        },

                                        format!("{:?}", called_function_definition.kind).to_lowercase()
                                    );
                                }

                                _ => ()
                            }
                        }
                        break;
                    }
                }
            }

            solidity::ast::Expression::MemberAccess(member_access) => {
                let referenced_declaration = match member_access.referenced_declaration {
                    Some(id) => id,
                    None => return Ok(())
                };

                for file in self.files.iter() {
                    if let Some((called_contract_definition, called_function_definition)) = file.function_and_contract_definition(referenced_declaration) {
                        if called_function_definition.state_mutability == solidity::ast::StateMutability::Payable {
                            match definition_node {
                                ContractDefinitionNode::FunctionDefinition(function_definition) => {
                                    println!(
                                        "\t{} {} {} makes a call to the {} payable {} {} without paying",

                                        format!("{:?}", function_definition.visibility),

                                        if function_definition.name.is_empty() {
                                            format!("{}", contract_definition.name)
                                        } else {
                                            format!("{}.{}", contract_definition.name, function_definition.name)
                                        },

                                        format!("{:?}", function_definition.kind).to_lowercase(),

                                        format!("{:?}", called_function_definition.visibility).to_lowercase(),

                                        if called_function_definition.name.is_empty() {
                                            format!("{}", called_contract_definition.name)
                                        } else {
                                            format!("{}.{}", called_contract_definition.name, called_function_definition.name)
                                        },

                                        format!("{:?}", called_function_definition.kind).to_lowercase()
                                    );
                                }

                                ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                                    println!(
                                        "\t{} {} modifier makes a call to the {} payable {} {} without paying",

                                        format!("{:?}", modifier_definition.visibility),

                                        if modifier_definition.name.is_empty() {
                                            format!("{}", contract_definition.name)
                                        } else {
                                            format!("{}.{}", contract_definition.name, modifier_definition.name)
                                        },

                                        format!("{:?}", called_function_definition.visibility).to_lowercase(),

                                        if called_function_definition.name.is_empty() {
                                            format!("{}", called_contract_definition.name)
                                        } else {
                                            format!("{}.{}", called_contract_definition.name, called_function_definition.name)
                                        },

                                        format!("{:?}", called_function_definition.kind).to_lowercase()
                                    );
                                }

                                _ => ()
                            }
                        }
                        break;
                    }
                }
            }

            _ => ()
        }

        Ok(())
    }
}
