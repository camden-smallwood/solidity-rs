pub struct InvalidUsingForDirectivesVisitor;

impl super::AstVisitor for InvalidUsingForDirectivesVisitor {
    fn visit_using_for_directive<'a>(&mut self, context: &mut super::UsingForDirectiveContext<'a>) -> std::io::Result<()> {
        let using_contract_id = match context.using_for_directive.library_name.referenced_declaration.as_ref() {
            Some(&id) => id,
            None => return Ok(())
        };

        let for_type_name = match context.using_for_directive.type_name.as_ref() {
            Some(type_name) => type_name,
            None => return Ok(())
        };

        let mut using_contract_definition = None;

        for source_unit in context.source_units.iter() {
            if let Some(contract_definition) = source_unit.contract_definition(using_contract_id) {
                using_contract_definition = Some(contract_definition);
                break;
            }
        }

        if let Some(using_contract_definition) = using_contract_definition {
            let mut function_found = false;

            for function_definition in using_contract_definition.function_definitions() {
                if let Some(parameter) = function_definition.parameters.parameters.first() {
                    if let Some(type_name) = parameter.type_name.as_ref() {
                        if type_name == for_type_name {
                            function_found = true;
                            break;
                        }
                    }
                }
            }

            if !function_found {
                println!(
                    "\tThe `{}` {} contains an invalid using-for directive: `{}`",

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    context.using_for_directive
                );
            }
        }

        Ok(())
    }
}
