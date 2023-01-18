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
        println!(
            "\t{} contains a requirement without a message: `{}`",
            contract_definition.definition_node_location(source_line, definition_node),
            function_call
        );
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
