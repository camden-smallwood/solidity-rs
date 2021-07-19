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
        // Attempt to retrieve the contract definition associated with the used library
        //

        let using_contract_definition = {
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
            
            using_contract_definition.unwrap()
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
        // Determine if the library contains any functions usable with the requested type
        //

        let mut usable_function_found = false;

        for function_definition in using_contract_definition.function_definitions() {
            //
            // Check to see if the parameter type matches the requested type
            //

            let parameter_type_name = match function_definition.parameters.parameters.first().map(|p| p.type_name.as_ref()) {
                Some(Some(type_name)) => type_name,
                _ => continue
            };

            if parameter_type_name == for_type_name {
                usable_function_found = true;
                break;
            }

            //
            // Check to see if the requested type inherits from the parameter type
            //

            let parameter_type_id = match parameter_type_name {
                TypeName::UserDefinedTypeName(UserDefinedTypeName { referenced_declaration, .. }) => referenced_declaration,
                _ => continue
            };

            if let Some(Some(linearized_base_contracts)) = for_contract_definition.map(|x| x.linearized_base_contracts.as_ref()) {
                if linearized_base_contracts.contains(parameter_type_id) {
                    usable_function_found = true;
                    break;
                }
            }
        }

        //
        // If the library does not contain any usable functions for the requested type, print a message
        //

        if !usable_function_found {
            println!(
                "\tL{}: The `{}` {} contains an invalid using-for directive: `{}`",

                context.current_source_unit.source_line(context.using_for_directive.src.as_str()).unwrap(),

                context.contract_definition.name,
                context.contract_definition.kind,

                context.using_for_directive
            );
        }

        Ok(())
    }
}
