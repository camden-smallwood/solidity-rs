use super::AstVisitor;
use solidity::ast::*;
use std::{collections::HashSet, io};

pub struct AssertUsageVisitor {
    reported_definitions: HashSet<NodeID>,
}

impl Default for AssertUsageVisitor {
    fn default() -> Self {
        Self {
            reported_definitions: HashSet::new(),
        }
    }
}

impl AstVisitor for AssertUsageVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut super::FunctionCallContext<'a, 'b>) -> io::Result<()> {
        let definition_id = match context.definition_node {
            &ContractDefinitionNode::FunctionDefinition(FunctionDefinition { id, .. })
            | &ContractDefinitionNode::ModifierDefinition(ModifierDefinition { id, .. }) => id,

            _ => return Ok(())
        };

        if let Expression::Identifier(Identifier { name, .. }) = context.function_call.expression.as_ref() {
            if name == "assert" && !self.reported_definitions.contains(&definition_id) {
                self.reported_definitions.insert(definition_id);

                match context.definition_node {
                    ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                        "\tThe {} {} in the `{}` {} contains `assert` usage",

                        function_definition.visibility,

                        if let FunctionKind::Constructor = function_definition.kind {
                            format!("{}", "constructor")
                        } else {
                            format!("`{}` {}", function_definition.name, function_definition.kind)
                        },

                        context.contract_definition.name,
                        context.contract_definition.kind,
                    ),

                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                        "\tThe {} `{}` modifier in the `{}` {} contains `assert` usage",
                        modifier_definition.visibility,
                        modifier_definition.name,
                        context.contract_definition.name,
                        context.contract_definition.kind,
                    ),

                    _ => ()
                }
            }
        }
        
        Ok(())
    }
}
