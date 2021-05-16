use crate::truffle;
use super::AstVisitor;
use std::io;

pub struct StateVariableShadowingVisitor<'a> {
    pub files: &'a [truffle::File],

}

impl<'a> StateVariableShadowingVisitor<'a> {
    pub fn new(files: &'a [truffle::File]) -> Self {
        Self { files }
    }
}

impl<'a> AstVisitor for StateVariableShadowingVisitor<'a> {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        function_definition: &solidity::ast::FunctionDefinition
    ) -> io::Result<()> {
        for &base_contract_id in contract_definition.linearized_base_contracts.iter() {
            let mut base_contract_definition = None;

            for file in self.files.iter() {
                if let Some(contract_definition) = file.contract_definition(base_contract_id) {
                    base_contract_definition = Some(contract_definition);
                    break;
                }
            }

            if let Some(base_contract_definition) = base_contract_definition {
                for variable_declaration in function_definition.parameters.parameters.iter() {
                    for base_variable_declaration in base_contract_definition.variable_declarations() {
                        if let solidity::ast::Visibility::Private = base_variable_declaration.visibility {
                            continue;
                        }

                        if variable_declaration.name == base_variable_declaration.name {
                            println!(
                                "\t{} {} {} has a {} {} parameter '{}' which shadows the {} {} {} state variable",

                                format!("{:?}", function_definition.visibility),

                                if function_definition.name.is_empty() {
                                    format!("{}", contract_definition.name)
                                } else {
                                    format!("{}.{}", contract_definition.name, function_definition.name)
                                },

                                format!("{:?}", function_definition.kind).to_lowercase(),

                                variable_declaration.type_descriptions.type_string.as_ref().unwrap(),

                                format!("{:?}", variable_declaration.storage_location).to_lowercase(),

                                variable_declaration.name,
                                
                                format!("{:?}", base_variable_declaration.visibility).to_lowercase(),

                                base_variable_declaration.type_descriptions.type_string.as_ref().unwrap(),

                                if base_variable_declaration.name.is_empty() {
                                    format!("{}", base_contract_definition.name)
                                } else {
                                    format!("{}.{}", base_contract_definition.name, base_variable_declaration.name)
                                },
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
