use super::{AstVisitor, FunctionDefinitionContext, LiteralContext};
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
                "\tL{}: {} {} {} contains large literals, which may be difficult to read",

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

    fn visit_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> {
        let definition_id = match context.definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => modifier_definition.id,
            _ => return Ok(())
        };

        if let Some(value) = context.literal.value.as_ref() {
            if value.chars().all(char::is_numeric) && (|n| (n > 6) && ((n % 3) != 0))(value.len()) {
                if !self.functions.contains(&definition_id) {
                    self.functions.insert(definition_id);
                }
            }
        }

        Ok(())
    }
}
