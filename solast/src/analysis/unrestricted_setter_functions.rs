use solidity::ast::*;
use std::io;

pub struct UnrestrictedSetterFunctionsVisitor;

impl UnrestrictedSetterFunctionsVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
    ) {
        println!(
            "\t{} is an unprotected setter function",
            contract_definition.definition_node_location(source_line, definition_node),
        );
    }
}

impl AstVisitor for UnrestrictedSetterFunctionsVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if let FunctionKind::Constructor = context.function_definition.kind {
            return Ok(())
        }

        if let Visibility::Private | Visibility::Internal = context.function_definition.visibility {
            return Ok(())
        }

        if let StateMutability::Pure | StateMutability::View = context.function_definition.state_mutability {
            return Ok(())
        }

        if !context.function_definition.modifiers.is_empty() {
            //
            // TODO: check for onlyOwner-like modifiers?
            //

            return Ok(())
        }

        match context.function_definition.body.as_ref() {
            Some(block) if !block.statements.is_empty() => {}
            _ => return Ok(())
        }

        for statement in context.function_definition.body.as_ref().unwrap().statements.iter() {
            match statement {
                Statement::ExpressionStatement(ExpressionStatement {
                    expression: Expression::Assignment(_),
                }) => continue,

                _ => return Ok(())
            }
        }

        self.print_message(
            context.contract_definition,
            context.definition_node,
            context.current_source_unit.source_line(context.function_definition.src.as_str())?,
        );

        Ok(())
    }
}
