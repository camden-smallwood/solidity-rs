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
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} calls a payable function without paying: `{}`",

                source_line,

                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    format!("{}", "constructor")
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },

                contract_definition.name,
                contract_definition.kind,

                expression
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The `{}` modifier in the `{}` {} calls a payable function without paying: `{}`",

                source_line,

                modifier_definition.name,

                contract_definition.name,
                contract_definition.kind,

                expression
            ),

            _ => {}
        }
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
