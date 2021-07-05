use super::{AstVisitor, FunctionDefinitionContext};
use solidity::ast::{FunctionKind, Visibility};
use std::io;

pub struct AbstractContractsVisitor;

impl AstVisitor for AbstractContractsVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if context.function_definition.kind != FunctionKind::Constructor {
            return Ok(())
        }

        if context.function_definition.visibility != Visibility::Internal {
            return Ok(())
        }

        match context.contract_definition.is_abstract {
            None | Some(false) => {
                println!(
                    "\tThe constructor of the `{}` contract is marked {} instead of marking `{}` as abstract",
                    context.contract_definition.name,
                    context.function_definition.visibility,
                    context.contract_definition.name,
                );
            }

            Some(true) => {
                if context.function_definition.visibility == Visibility::Internal {
                    println!(
                        "\tThe constructor of the `{}` contract is marked {} when `{}` is already marked as abstract",
                        context.contract_definition.name,
                        context.function_definition.visibility,
                        context.contract_definition.name,
                    );
                }
            }
        }

        Ok(())
    }
}
