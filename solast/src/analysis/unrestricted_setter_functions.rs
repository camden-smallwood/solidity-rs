use super::{AstVisitor, FunctionDefinitionContext};
use std::io;

use solidity::ast::{
    Expression, ExpressionStatement, FunctionKind, StateMutability, Statement, Visibility
};

pub struct UnrestrictedSetterFunctionsVisitor;

impl AstVisitor for UnrestrictedSetterFunctionsVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if let FunctionKind::Constructor = context.function_definition.kind {
            return Ok(())
        }

        if let Visibility::Private | Visibility::Internal = context.function_definition.visibility {
            return Ok(())
        }

        if let StateMutability::Pure | StateMutability::View = context.function_definition.state_mutability {
            return Ok(())
        }

        if !context.function_definition.modifiers.is_empty() {
            //
            // TODO: check for onlyOwner-like modifiers?
            //

            return Ok(())
        }

        match context.function_definition.body.as_ref() {
            Some(block) if !block.statements.is_empty() => {}
            _ => return Ok(())
        }

        for statement in context.function_definition.body.as_ref().unwrap().statements.iter() {
            match statement {
                Statement::ExpressionStatement(ExpressionStatement {
                    expression: Expression::Assignment(_),
                }) => continue,

                _ => return Ok(())
            }
        }

        println!(
            "\t{} {} {} ({}) is an unprotected setter function",

            format!("{:?}", context.function_definition.visibility),

            if let FunctionKind::Constructor = context.function_definition.kind {
                format!("{}", context.contract_definition.name)
            } else {
                format!("{}.{}", context.contract_definition.name, context.function_definition.name)
            },

            context.function_definition.kind,

            context.function_definition.src
        );

        Ok(())
    }
}
