use super::AstVisitor;
use std::io;

use solidity::ast::{ContractDefinition, ContractDefinitionNode, Expression, ExpressionStatement, FunctionDefinition, FunctionKind, SourceUnit, StateMutability, Statement, Visibility};

pub struct UnrestrictedSetterFunctionsVisitor;

impl AstVisitor for UnrestrictedSetterFunctionsVisitor {
    fn visit_function_definition(
        &mut self,
        _source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        _definition_node: &ContractDefinitionNode,
        function_definition: &FunctionDefinition,
    ) -> io::Result<()> {
        if let FunctionKind::Constructor = function_definition.kind {
            return Ok(())
        }

        if let Visibility::Private | Visibility::Internal = function_definition.visibility {
            return Ok(())
        }

        if let StateMutability::Pure | StateMutability::View = function_definition.state_mutability {
            return Ok(())
        }

        if !function_definition.modifiers.is_empty() {
            //
            // TODO: check for onlyOwner-like modifiers?
            //

            return Ok(())
        }

        if function_definition.body.is_none() {
            return Ok(())
        }

        for statement in function_definition.body.as_ref().unwrap().statements.iter() {
            match statement {
                Statement::ExpressionStatement(ExpressionStatement {
                    expression: Expression::Assignment(_),
                }) => continue,

                _ => return Ok(())
            }
        }

        println!(
            "\t{} {} {} is an unprotected setter function",

            format!("{:?}", function_definition.visibility),

            if let FunctionKind::Constructor = function_definition.kind {
                format!("{}", contract_definition.name)
            } else {
                format!("{}.{}", contract_definition.name, function_definition.name)
            },

            function_definition.kind
        );

        Ok(())
    }
}
