use solidity::ast::*;
use std::io;
pub struct SafeERC20FunctionsVisitor;

impl SafeERC20FunctionsVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        unsafe_name: &str,
        safe_name: &str
    ) {
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} uses `ERC20.{}` instead of `SafeERC20.{}`",
    
                source_line,
    
                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    format!("{}", "constructor")
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },
    
                contract_definition.name,
                contract_definition.kind,
    
                unsafe_name,
    
                safe_name,
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The `{}` modifier in the `{}` {} uses `ERC20.{}` instead of `SafeERC20.{}`",

                source_line,

                modifier_definition.name,

                contract_definition.name,
                contract_definition.kind,

                unsafe_name,
    
                safe_name,
            ),

            _ => {}
        }
    }
}

impl AstVisitor for SafeERC20FunctionsVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if context.contract_definition.name == "SafeERC20" {
            return Ok(())
        }

        for referenced_declaration in context.function_call.expression.referenced_declarations() {
            for source_unit in context.source_units.iter() {
                let (called_contract_definition, called_function_definition) = match source_unit.function_and_contract_definition(referenced_declaration) {
                    Some((contract_definition, function_definition)) => (contract_definition, function_definition),
                    None => continue
                };
            
                match called_contract_definition.name.to_ascii_lowercase().as_str() {
                    "erc20" | "ierc20" | "erc20interface" => {}
                    _ => return Ok(())
                }

                match called_function_definition.name.as_str() {
                    "transfer" => self.print_message(
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(context.function_call.src.as_str())?,
                        "transfer",
                        "safeTransfer"
                    ),

                    "transferFrom" => self.print_message(
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(context.function_call.src.as_str())?,
                        "transferFrom",
                        "safeTransferFrom"
                    ),

                    "approve" => self.print_message(
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(context.function_call.src.as_str())?,
                        "approve",
                        "safeApprove"
                    ),

                    _ => {}
                }

                break;
            }
        }

        Ok(())
    }
}
