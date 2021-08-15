use solidity::ast::*;
use std::io;

pub struct SecureEtherTransferVisitor;

impl SecureEtherTransferVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        expression: &dyn std::fmt::Display
    ) {
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} ignores the Secure-Ether-Transfer pattern: `{}`",
    
                source_line,
    
                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    format!("{}", "constructor")
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },
    
                contract_definition.name,
                contract_definition.kind,
    
                expression
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The `{}` modifier in the `{}` {} ignores the Secure-Ether-Transfer pattern: `{}`",

                source_line,

                modifier_definition.name,

                contract_definition.name,
                contract_definition.kind,
    
                expression
            ),

            _ => {}
        }
    }
}

impl AstVisitor for SecureEtherTransferVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if let Expression::MemberAccess(member_access) = context.function_call.expression.as_ref() {
            if let Some(TypeDescriptions { type_string: Some(type_string), .. }) = member_access.expression.as_ref().type_descriptions() {
                match type_string.as_str() {
                    "address" | "address payable" => {}
                    _ => return Ok(())
                }
            }

            match member_access.member_name.as_str() {
                "transfer" | "send" => {}
                _ => return Ok(())
            }
            
            if member_access.referenced_declaration.is_none() || member_access.referenced_declaration.map(|id| id == 0).unwrap_or(false) {
                self.print_message(
                    context.contract_definition,
                    context.definition_node,
                    context.current_source_unit.source_line(context.function_call.src.as_str())?,
                    context.function_call
                );
            }
        }

        Ok(())
    }

    fn visit_function_call_options<'a, 'b>(&mut self, context: &mut FunctionCallOptionsContext<'a, 'b>) -> io::Result<()> {
        if let Expression::MemberAccess(member_access) = context.function_call_options.expression.as_ref() {
            if let Some(TypeDescriptions { type_string: Some(type_string), .. }) = member_access.expression.as_ref().type_descriptions() {
                match type_string.as_str() {
                    "address" | "address payable" => {}
                    _ => return Ok(())
                }
            }

            match member_access.member_name.as_str() {
                "transfer" | "send" => {}
                _ => return Ok(())
            }
            
            if member_access.referenced_declaration.is_none() || member_access.referenced_declaration.map(|id| id == 0).unwrap_or(false) {
                self.print_message(
                    context.contract_definition,
                    context.definition_node,
                    context.current_source_unit.source_line(context.function_call_options.src.as_str())?,
                    context.function_call_options
                );
            }
        }

        Ok(())
    }
}
