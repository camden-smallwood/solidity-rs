use super::{AstVisitor, FunctionDefinitionContext};
use std::io;

pub struct StateVariableShadowingVisitor;

impl AstVisitor for StateVariableShadowingVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        let contract_ids = match context.contract_definition.linearized_base_contracts.as_ref() {
            Some(contract_ids) => contract_ids,
            None => return Ok(()),
        };
        
        for &base_contract_id in contract_ids.iter() {
            let mut base_contract_definition = None;

            for source_unit in context.source_units.iter() {
                if let Some(contract_definition) = source_unit.contract_definition(base_contract_id) {
                    base_contract_definition = Some(contract_definition);
                    break;
                }
            }

            if let Some(base_contract_definition) = base_contract_definition {
                for variable_declaration in context.function_definition.parameters.parameters.iter() {
                    for base_variable_declaration in base_contract_definition.variable_declarations() {
                        if let solidity::ast::Visibility::Private = base_variable_declaration.visibility {
                            continue;
                        }

                        if variable_declaration.name == base_variable_declaration.name {
                            println!(
                                "\t{} {} {} has a {} {} parameter '{}' which shadows the {} {} {} state variable",

                                format!("{:?}", context.function_definition.visibility),

                                if context.function_definition.name.is_empty() {
                                    format!("{}", context.contract_definition.name)
                                } else {
                                    format!("{}.{}", context.contract_definition.name, context.function_definition.name)
                                },

                                context.function_definition.kind,

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
