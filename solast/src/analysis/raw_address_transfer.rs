use super::{AstVisitor, FunctionDefinitionContext};
use solidity::ast::*;
use std::{collections::HashMap, io};

pub struct RawAddressTransferVisitor {
    functions_transfer: HashMap<NodeID, usize>,
}

impl Default for RawAddressTransferVisitor {
    fn default() -> Self {
        Self {
            functions_transfer: HashMap::new(),
        }
    }
}

impl AstVisitor for RawAddressTransferVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if !self.functions_transfer.contains_key(&context.function_definition.id) {
            self.functions_transfer.insert(context.function_definition.id, 0);
        }

        Ok(())
    }

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if let Some(&transfer_count) = self.functions_transfer.get(&context.function_definition.id) {
            if transfer_count > 0 {
                println!(
                    "\t{} {} {} performs {}",

                    format!("{:?}", context.function_definition.visibility),

                    if context.function_definition.name.is_empty() {
                        format!("{}", context.contract_definition.name)
                    } else {
                        format!("{}.{}", context.contract_definition.name, context.function_definition.name)
                    },

                    context.function_definition.kind,
                    
                    if transfer_count == 1 {
                        "a raw address transfer"
                    } else {
                        "raw address transfers"
                    }
                );
            }
        }

        Ok(())
    }

    fn visit_function_call<'a, 'b>(&mut self, context: &mut super::FunctionCallContext<'a, 'b>) -> io::Result<()> {
        let definition_id = match context.definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        if let solidity::ast::Expression::MemberAccess(member_access) = context.function_call.expression.as_ref() {
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
                *self.functions_transfer.get_mut(&definition_id).unwrap() += 1;
            }
        }

        Ok(())
    }

    fn visit_function_call_options<'a, 'b>(&mut self, context: &mut super::FunctionCallOptionsContext<'a, 'b>) -> io::Result<()> {
        let definition_id = match context.definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        if let solidity::ast::Expression::MemberAccess(member_access) = context.function_call_options.expression.as_ref() {
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
                *self.functions_transfer.get_mut(&definition_id).unwrap() += 1;
            }
        }

        Ok(())
    }
}
