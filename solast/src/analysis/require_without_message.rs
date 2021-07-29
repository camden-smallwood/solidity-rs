use solidity::ast::*;
use std::{collections::HashMap, io};

pub struct RequireWithoutMessageVisitor {
    requirement_counts: HashMap<NodeID, usize>,
}

impl Default for RequireWithoutMessageVisitor {
    fn default() -> Self {
        Self {
            requirement_counts: HashMap::new(),
        }
    }
}

impl AstVisitor for RequireWithoutMessageVisitor {
    fn leave_modifier_definition<'a>(&mut self, context: &mut ModifierDefinitionContext<'a>) -> io::Result<()> {
        if let Some(&requirement_count) = self.requirement_counts.get(&context.modifier_definition.id) {
            println!(
                "\tL{}: {} {} modifier has {} without {}",

                context.current_source_unit.source_line(context.modifier_definition.src.as_str())?,

                format!("{:?}", context.modifier_definition.visibility),

                if context.modifier_definition.name.is_empty() {
                    format!("{}", context.contract_definition.name)
                } else {
                    format!("{}.{}", context.contract_definition.name, context.modifier_definition.name)
                },

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

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if let Some(&requirement_count) = self.requirement_counts.get(&context.function_definition.id) {
            println!(
                "\tL{}: {} {} {} has {} without {}",

                context.current_source_unit.source_line(context.function_definition.src.as_str())?,

                format!("{:?}", context.function_definition.visibility),

                if context.function_definition.name.is_empty() {
                    format!("{}", context.contract_definition.name)
                } else {
                    format!("{}.{}", context.contract_definition.name, context.function_definition.name)
                },

                context.function_definition.kind,

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

    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        let definition_id = match context.definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        match context.function_call.expression.as_ref() {
            solidity::ast::Expression::Identifier(expr) if expr.name == "require" => (),
            _ => return Ok(())
        }

        if context.function_call.arguments.len() < 2 {
            if !self.requirement_counts.contains_key(&definition_id) {
                self.requirement_counts.insert(definition_id, 0);
            }

            *self.requirement_counts.get_mut(&definition_id).unwrap() += 1;
        }

        Ok(())
    }
}
