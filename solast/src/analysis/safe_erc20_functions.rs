use solidity::ast::*;
use std::{collections::HashMap, io};

struct FunctionInfo {
    transfer: bool,
    transfer_from: bool,
    approve: bool,
}

pub struct SafeERC20FunctionsVisitor {
    functions: HashMap<NodeID, FunctionInfo>,
}

impl Default for SafeERC20FunctionsVisitor {
    fn default() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
}

impl AstVisitor for SafeERC20FunctionsVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if !self.functions.contains_key(&context.function_definition.id) {
            self.functions.insert(
                context.function_definition.id,
                FunctionInfo {
                    transfer: false,
                    transfer_from: false,
                    approve: false,
                },
            );
        }

        Ok(())
    }

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        let function_info = self.functions.get(&context.function_definition.id).unwrap();

        if function_info.transfer {
            println!(
                "\tL{}: {} {} {} uses `ERC20.transfer` instead of `SafeERC20.safeTransfer`",

                context.current_source_unit.source_line(context.function_definition.src.as_str()).unwrap(),

                format!("{:?}", context.function_definition.visibility),

                if context.function_definition.name.is_empty() {
                    format!("{}", context.contract_definition.name)
                } else {
                    format!("{}.{}", context.contract_definition.name, context.function_definition.name)
                },

                context.function_definition.kind
            );
        }

        if function_info.transfer_from {
            println!(
                "\tL{}: {} {} {} uses `ERC20.transferFrom` instead of `SafeERC20.safeTransferFrom`",

                context.current_source_unit.source_line(context.function_definition.src.as_str()).unwrap(),

                format!("{:?}", context.function_definition.visibility),

                if context.function_definition.name.is_empty() {
                    format!("{}", context.contract_definition.name)
                } else {
                    format!("{}.{}", context.contract_definition.name, context.function_definition.name)
                },

                context.function_definition.kind
            );
        }

        if function_info.approve {
            println!(
                "\tL{}: {} {} {} uses `ERC20.approve` instead of `SafeERC20.safeApprove`",

                context.current_source_unit.source_line(context.function_definition.src.as_str()).unwrap(),

                format!("{:?}", context.function_definition.visibility),

                if context.function_definition.name.is_empty() {
                    format!("{}", context.contract_definition.name)
                } else {
                    format!("{}.{}", context.contract_definition.name, context.function_definition.name)
                },

                context.function_definition.kind
            );
        }

        Ok(())
    }

    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        let definition_id = match context.definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        if context.contract_definition.name == "SafeERC20" {
            return Ok(());
        }

        let function_info = match self.functions.get_mut(&definition_id) {
            Some(function_info) => function_info,
            None => return Ok(())
        };

        for referenced_declaration in context.function_call.expression.referenced_declarations() {
            for source_unit in context.source_units.iter() {
                if let Some((called_contract_definition, called_function_definition)) = source_unit.function_and_contract_definition(referenced_declaration) {
                    if let "erc20" | "ierc20" | "erc20interface" = called_contract_definition.name.to_ascii_lowercase().as_str() {
                        match called_function_definition.name.as_str() {
                            "transfer" => function_info.transfer = true,
                            "transferFrom" => function_info.transfer_from = true,
                            "approve" => function_info.approve = true,

                            _ => {}
                        }
                    }

                    break;
                }
            }
        }

        Ok(())
    }
}
