use solidity::ast::*;
use std::io;

pub struct UnpaidPayableFunctionsVisitor;

impl UnpaidPayableFunctionsVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        expression: &dyn std::fmt::Display
    ) {
        println!(
            "\t{} calls a payable function without paying: `{}`",
            contract_definition.definition_node_location(source_line, definition_node),
            expression
        );
    }
}

impl AstVisitor for UnpaidPayableFunctionsVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        match context.function_call.expression.as_ref() {
            solidity::ast::Expression::Identifier(identifier) => {
                for source_unit in context.source_units.iter() {
                    if let Some(FunctionDefinition {
                        state_mutability: StateMutability::Payable,
                        ..
                    }) = source_unit.function_definition(identifier.referenced_declaration) {
                        self.print_message(
                            context.contract_definition,
                            context.definition_node,
                            context.current_source_unit.source_line(context.function_call.src.as_str())?,
                            context.function_call,
                        );
                        break;
                    }
                }
            }

            solidity::ast::Expression::MemberAccess(member_access) => {
                let referenced_declaration = match member_access.referenced_declaration {
                    Some(id) => id,
                    None => return Ok(()),
                };

                for source_unit in context.source_units.iter() {
                    if let Some(FunctionDefinition {
                        state_mutability: StateMutability::Payable,
                        ..
                    }) = source_unit.function_definition(referenced_declaration) {
                        self.print_message(
                            context.contract_definition,
                            context.definition_node,
                            context.current_source_unit.source_line(context.function_call.src.as_str())?,
                            context.function_call,
                        );
                        break;
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }
}
