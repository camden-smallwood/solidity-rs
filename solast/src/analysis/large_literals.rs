use solidity::ast::*;
use std::io;

pub struct LargeLiteralsVisitor;

impl LargeLiteralsVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        literal: &Literal
    ) {
        println!(
            "\t{} contains a large literal, which may be difficult to read: `{}`",
            contract_definition.definition_node_location(source_line, definition_node),
            literal
        );
    }
}

impl AstVisitor for LargeLiteralsVisitor {
    fn visit_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> {
        if let Some(value) = context.literal.value.as_ref() {
            let n = value.len();

            if value.chars().all(char::is_numeric) && (n > 6) && ((n % 3) != 0) {
                self.print_message(
                    context.contract_definition,
                    context.definition_node,
                    context.current_source_unit.source_line(context.literal.src.as_str())?,
                    context.literal
                );
            }
        }

        Ok(())
    }
}
