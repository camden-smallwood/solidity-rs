use solidity::ast::*;
use std::io;

pub struct RequireWithoutMessageVisitor;

impl RequireWithoutMessageVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        function_call: &FunctionCall
    ) {
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} contains a requirement without a message: `{}`",
    
                source_line,
    
                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    "constructor".to_string()
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },
    
                contract_definition.name,
                contract_definition.kind,
    
                function_call
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The `{}` modifier in the `{}` {} contains a requirement without a message: `{}`",

                source_line,

                modifier_definition.name,

                contract_definition.name,
                contract_definition.kind,
    
                function_call
            ),

            _ => {}
        }
    }
}

impl AstVisitor for RequireWithoutMessageVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if let Expression::Identifier(Identifier { name, .. }) = context.function_call.expression.as_ref() {
            if name == "require" && context.function_call.arguments.len() < 2 {
                self.print_message(
                    context.contract_definition,
                    context.definition_node,
                    context.current_source_unit.source_line(context.function_call.src.as_str())?,
                    context.function_call
                );
            }
        }

        Ok(())
    }
}
