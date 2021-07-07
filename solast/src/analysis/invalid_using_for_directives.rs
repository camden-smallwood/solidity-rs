use super::{AstVisitor, UsingForDirectiveContext};

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
            if let Some(parameter) = function_definition.parameters.parameters.first() {
                if let Some(type_name) = parameter.type_name.as_ref() {
                    if type_name == for_type_name {
                        usable_function_found = true;
                        break;
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
