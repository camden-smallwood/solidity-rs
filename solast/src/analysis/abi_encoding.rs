use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct AbiEncodingVisitor {
    declaration_type_names: HashMap<NodeID, TypeName>
}

impl AstVisitor for AbiEncodingVisitor {
    fn visit_variable_declaration<'a, 'b>(&mut self, context: &mut VariableDeclarationContext<'a, 'b>) -> std::io::Result<()> {
        //
        // Store the type of any variable declarations
        //

        if let Some(type_name) = context.variable_declaration.type_name.as_ref() {
            if self.declaration_type_names.contains_key(&context.variable_declaration.id) {
                return Ok(())
            }

            self.declaration_type_names.insert(context.variable_declaration.id, type_name.clone());
        }

        Ok(())
    }

    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> std::io::Result<()> {
        //
        // Only check for calls to abi.encodePacked(...)
        //

        if let Expression::MemberAccess(MemberAccess { expression, member_name, .. }) = context.function_call.expression.as_ref() {
            if let Expression::Identifier(Identifier { name, .. }) = expression.as_ref() {
                if name != "abi" || member_name != "encodePacked" {
                    return Ok(())
                }
            } else {
                return Ok(())
            }
        } else {
            return Ok(())
        }

        //
        // Only check if multiple arguments are supplied: abi.encodePacked(as, bs, ...)
        //

        if context.function_call.arguments.len() < 2 {
            return Ok(())
        }

        //
        // Determine if any parameters contain any variably-sized arrays
        //

        let mut any_arguments_variably_sized = false;

        for expression in context.function_call.arguments.iter() {
            if any_arguments_variably_sized {
                break;
            }

            for referenced_declaration in expression.referenced_declarations() {
                if let Some(TypeName::ArrayTypeName(ArrayTypeName { length: None, .. })) = self.declaration_type_names.get(&referenced_declaration) {
                    any_arguments_variably_sized = true;
                    break;
                }
            }
        }

        //
        // If so, print a message warning about potential hash collisions
        //

        if any_arguments_variably_sized {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tL{}: The {} {} in the `{}` {} contains the potential for hash collisions: `{}`",

                    context.current_source_unit.source_line(context.function_call.src.as_str())?,

                    function_definition.visibility,
    
                    if let FunctionKind::Constructor = function_definition.kind {
                        "constructor".to_string()
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },
    
                    context.contract_definition.name,
                    context.contract_definition.kind,
    
                    context.function_call
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tL{}: The `{}` modifier in the `{}` {} contains the potential for hash collisions: `{}`",
    
                    context.current_source_unit.source_line(context.function_call.src.as_str())?,

                    modifier_definition.name,
    
                    context.contract_definition.name,
                    context.contract_definition.kind,
    
                    context.function_call
                ),

                _ => {}
            }
        }

        Ok(())
    }
}
