use super::AstVisitor;
use crate::truffle;
use solidity::ast::NodeID;
use std::{collections::HashMap, io};

pub struct RequireWithoutMessageVisitor<'a> {
    pub files: &'a [truffle::File],
    pub function_requirement_counts: HashMap<NodeID, usize>,
}

impl<'a> RequireWithoutMessageVisitor<'a> {
    pub fn new(files: &'a [truffle::File]) -> Self {
        Self {
            files,
            function_requirement_counts: HashMap::new(),
        }
    }
}

impl AstVisitor for RequireWithoutMessageVisitor<'_> {
    fn leave_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if let Some(&requirement_count) = self
            .function_requirement_counts
            .get(&function_definition.id)
        {
            println!(
                "\t{} {} {} has {} without {}",
                format!("{:?}", function_definition.visibility),
                if function_definition.name.is_empty() {
                    format!("{}", contract_definition.name)
                } else {
                    format!("{}.{}", contract_definition.name, function_definition.name)
                },
                format!("{:?}", function_definition.kind).to_lowercase(),
                if requirement_count > 1 {
                    "requirements"
                } else {
                    "a requirement"
                },
                if requirement_count > 1 {
                    "messages"
                } else {
                    "a message"
                }
            );
        }

        Ok(())
    }

    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        function_definition: Option<&'a solidity::ast::FunctionDefinition>,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        function_call: &'a solidity::ast::FunctionCall,
    ) -> io::Result<()> {
        let function_definition = match function_definition {
            Some(function_definition) => function_definition,
            None => return Ok(()),
        };

        match function_call.expression.as_ref() {
            solidity::ast::Expression::Identifier(expr) if expr.name == "require" => (),
            _ => return Ok(()),
        }

        if function_call.arguments.len() < 2 {
            if !self
                .function_requirement_counts
                .contains_key(&function_definition.id)
            {
                self.function_requirement_counts
                    .insert(function_definition.id, 0);
            }

            *self
                .function_requirement_counts
                .get_mut(&function_definition.id)
                .unwrap() += 1;
        }

        Ok(())
    }
}
