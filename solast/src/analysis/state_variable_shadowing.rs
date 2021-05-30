use super::AstVisitor;
use solidity::ast::SourceUnit;
use std::io;

pub struct StateVariableShadowingVisitor<'a> {
    pub source_units: &'a [SourceUnit],

}

impl<'a> StateVariableShadowingVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self { source_units }
    }
}

impl<'a> AstVisitor for StateVariableShadowingVisitor<'a> {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition
    ) -> io::Result<()> {
        for &base_contract_id in contract_definition.linearized_base_contracts.iter() {
            let mut base_contract_definition = None;

            for source_unit in self.source_units.iter() {
                if let Some(contract_definition) = source_unit.contract_definition(base_contract_id) {
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
