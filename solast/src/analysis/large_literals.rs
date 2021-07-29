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
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} contains a large literal, which may be difficult to read: `{}`",
    
                source_line,
    
                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    format!("{}", "constructor")
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },
    
                contract_definition.name,
                contract_definition.kind,
    
                literal
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The `{}` modifier in the `{}` {} contains a large literal, which may be difficult to read: `{}`",

                source_line,

                modifier_definition.name,

                contract_definition.name,
                contract_definition.kind,
    
                literal
            ),

            _ => {}
        }
    }
}

impl AstVisitor for LargeLiteralsVisitor {
    fn visit_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> {
        if let Some(value) = context.literal.value.as_ref() {
            if value.chars().all(char::is_numeric) && (|n| (n > 6) && ((n % 3) != 0))(value.len()) {
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
