use super::{AstVisitor, UsingForDirectiveContext};
use solidity::ast::*;

pub struct InvalidUsingForDirectivesVisitor;

impl AstVisitor for InvalidUsingForDirectivesVisitor {
    fn visit_using_for_directive<'a>(&mut self, context: &mut UsingForDirectiveContext<'a>) -> std::io::Result<()> {
        //
        // Get the identifier of the contract definition associated with the used library
        //

        let using_contract_id = match context.using_for_directive.library_name.referenced_declaration.as_ref() {
            Some(&id) => id,
            None => return Ok(())
        };

        //
        // Get the type name of the requested type to use the library for
        //

        let for_type_name = match context.using_for_directive.type_name.as_ref() {
            Some(type_name) => type_name,
            None => return Ok(())
        };

        //
        // Get the contract definition of the requested type to use the library for (if any)
        //

        let mut for_contract_definition = None;

        if let &TypeName::UserDefinedTypeName(UserDefinedTypeName { referenced_declaration, .. }) = for_type_name {
            for source_unit in context.source_units.iter() {
                if let Some(contract_definition) = source_unit.contract_definition(referenced_declaration) {
                    for_contract_definition = Some(contract_definition);
                    break;
                }
            }
        }

        //
        // Attempt to retrieve the contract definition associated with the used library
        //

        let mut using_contract_definition = None;

        for source_unit in context.source_units.iter() {
            if let Some(contract_definition) = source_unit.contract_definition(using_contract_id) {
                using_contract_definition = Some(contract_definition);
                break;
            }
        }

        if using_contract_definition.is_none() {
            return Ok(())
        }

        let using_contract_definition = using_contract_definition.unwrap();

        //
        // Determine if the library contains a usable function for the requested type
        //

        let mut usable_function_found = false;

        for function_definition in using_contract_definition.function_definitions() {
            let parameter = match function_definition.parameters.parameters.first() {
                Some(parameter) => parameter,
                None => continue
            };

            let type_name = match parameter.type_name.as_ref() {
                Some(type_name) => type_name,
                None => continue
            };

            if type_name == for_type_name {
                usable_function_found = true;
                break;
            }

            if let TypeName::UserDefinedTypeName(UserDefinedTypeName { referenced_declaration, .. }) = type_name {
                if let Some(for_contract_definition) = for_contract_definition {
                    if let Some(linearized_base_contracts) = for_contract_definition.linearized_base_contracts.as_ref() {
                        if linearized_base_contracts.contains(referenced_declaration) {
                            usable_function_found = true;
                            break;
                        }
                    }
                }
            }
        }

        //
        // If the library does not contain any usable functions for the requested type, print a message
        //

        if !usable_function_found {
            println!(
                "\tThe `{}` {} contains an invalid using-for directive: `{}`",

                context.contract_definition.name,
                context.contract_definition.kind,

                context.using_for_directive
            );
        }

        Ok(())
    }
}
