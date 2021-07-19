use solidity::ast::*;
use std::collections::HashSet;

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
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> std::io::Result<()> {
        //
        // Get the identifier associated with the function or modifier containing the function call
        //

        let definition_id = match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(FunctionDefinition { id, .. }) |
            ContractDefinitionNode::ModifierDefinition(ModifierDefinition { id, .. }) => id.clone(),

            _ => return Ok(())
        };

        //
        // Check if the expression of function call is the "assert" identifier
        //

        let is_assert = match context.function_call.expression.as_ref() {
            Expression::Identifier(Identifier { name, .. }) => name == "assert",
            _ => false
        };

        if !is_assert {
            return Ok(())
        }

        //
        // Don't display multiple messages for the same function or modifier
        //

        if self.reported_definitions.contains(&definition_id) {
            return Ok(())
        }

        self.reported_definitions.insert(definition_id);

        //
        // Print a message about the assert usage
        //

        match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} contains `assert` usage",

                context.current_source_unit.source_line(context.function_call.src.as_str()).unwrap(),

                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    format!("{}", "constructor")
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },

                context.contract_definition.name,
                context.contract_definition.kind
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The {} `{}` modifier in the `{}` {} contains `assert` usage",

                context.current_source_unit.source_line(context.function_call.src.as_str()).unwrap(),

                modifier_definition.visibility,
                modifier_definition.name,

                context.contract_definition.name,
                context.contract_definition.kind
            ),

            _ => {}
        }
        
        Ok(())
    }
}
