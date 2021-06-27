use super::{AstVisitor, FunctionDefinitionContext};
use solidity::ast::NodeID;
use std::{collections::HashSet, io};

pub struct LargeLiteralsVisitor {
    functions: HashSet<NodeID>,
}

impl Default for LargeLiteralsVisitor {
    fn default() -> Self {
        Self {
            functions: HashSet::new(),
        }
    }
}

impl AstVisitor for LargeLiteralsVisitor {
    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if self.functions.contains(&context.function_definition.id) {
            println!(
                "\t{} {} {} contains large literals, which may be difficult to read",

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

    fn visit_literal(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
        definition_node: &solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&solidity::ast::Block>,
        _statement: Option<&solidity::ast::Statement>,
        literal: &solidity::ast::Literal,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => modifier_definition.id,
            _ => return Ok(())
        };

        if let Some(value) = literal.value.as_ref() {
            if value.chars().all(char::is_numeric) && (|n| (n > 6) && ((n % 3) != 0))(value.len()) {
                if !self.functions.contains(&definition_id) {
                    self.functions.insert(definition_id);
                }
            }
        }

        Ok(())
    }
}
