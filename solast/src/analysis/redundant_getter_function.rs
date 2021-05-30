use super::AstVisitor;
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

impl<'a> AstVisitor for RedundantGetterFunctionVisitor<'a> {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if function_definition.name.is_empty() || function_definition.body.is_none() {
            return Ok(());
        }

        if function_definition.return_parameters.parameters.len() != 1 {
            return Ok(());
        }

        if function_definition.visibility != solidity::ast::Visibility::Public {
            return Ok(());
        }

        let statements = function_definition
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
                match contract_definition.variable_declaration(identifier.referenced_declaration) {
                    Some(variable_declaration) => variable_declaration,
                    None => return Ok(()),
                }
            }
            _ => return Ok(()),
        };

        if (variable_declaration.name != function_definition.name)
            && !(variable_declaration.name.starts_with('_')
                && variable_declaration.name[1..] == function_definition.name)
        {
            return Ok(());
        }

        println!(
            "\t{} {}.{} {} is a redundant getter function for the {} {}.{} state variable",
            format!("{:?}", function_definition.visibility),
            contract_definition.name,
            function_definition.name,
            format!("{:?}", function_definition.kind).to_lowercase(),
            format!("{:?}", variable_declaration.visibility).to_lowercase(),
            contract_definition.name,
            variable_declaration.name,
        );

        Ok(())
    }
}
