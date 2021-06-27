use super::{AstVisitor, FunctionDefinitionContext};
use solidity::ast::SourceUnit;
use std::io;

pub struct RedundantGetterFunctionVisitor<'a> {
    pub source_units: &'a [SourceUnit],
}

impl<'a> RedundantGetterFunctionVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self { source_units }
    }
}

impl AstVisitor for RedundantGetterFunctionVisitor<'_> {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if context.function_definition.name.is_empty() || context.function_definition.body.is_none() {
            return Ok(());
        }

        if context.function_definition.return_parameters.parameters.len() != 1 {
            return Ok(());
        }

        if context.function_definition.visibility != solidity::ast::Visibility::Public {
            return Ok(());
        }

        let statements = context.function_definition
            .body
            .as_ref()
            .unwrap()
            .statements
            .as_slice();

        if statements.len() != 1 {
            return Ok(());
        }

        let return_statement = match &statements[0] {
            solidity::ast::Statement::Return(return_statement) => return_statement,
            _ => return Ok(()),
        };

        let variable_declaration = match return_statement.expression.as_ref() {
            Some(solidity::ast::Expression::Identifier(identifier)) => {
                match context.contract_definition.variable_declaration(identifier.referenced_declaration) {
                    Some(variable_declaration) => variable_declaration,
                    None => return Ok(()),
                }
            }
            _ => return Ok(()),
        };

        if (variable_declaration.name != context.function_definition.name)
            && !(variable_declaration.name.starts_with('_')
                && variable_declaration.name[1..] == context.function_definition.name)
        {
            return Ok(());
        }

        println!(
            "\t{} {}.{} {} is a redundant getter function for the {} {}.{} state variable",
            format!("{:?}", context.function_definition.visibility),
            context.contract_definition.name,
            context.function_definition.name,
            context.function_definition.kind,
            variable_declaration.visibility,
            context.contract_definition.name,
            variable_declaration.name,
        );

        Ok(())
    }
}
