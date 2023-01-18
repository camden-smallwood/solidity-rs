use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::io;

#[derive(Default)]
pub struct ExternalCallsInLoopVisitor {
    loop_ids: Vec<NodeID>,
    function_calls: Vec<FunctionCall>,
}

impl ExternalCallsInLoopVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        expression: &dyn std::fmt::Display
    ) {
        println!(
            "\t{} makes an external call inside a loop: `{}`",
            contract_definition.definition_node_location(source_line, definition_node),
            expression
        );
    }
}

impl AstVisitor for ExternalCallsInLoopVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        self.function_calls.push(context.function_call.clone());

        Ok(())
    }

    fn leave_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        match self.function_calls.pop() {
            Some(function_call) => {
                if function_call.id != context.function_call.id {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid function call id stack"));
                }
            },

            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Not currently in a function call"))
        }

        Ok(())
    }

    fn visit_for_statement<'a, 'b>(&mut self, context: &mut ForStatementContext<'a, 'b>) -> io::Result<()> {
        self.loop_ids.push(context.for_statement.id);

        Ok(())
    }

    fn leave_for_statement<'a, 'b>(&mut self, context: &mut ForStatementContext<'a, 'b>) -> io::Result<()> {
        match self.loop_ids.pop() {
            Some(loop_id) => {
                if loop_id != context.for_statement.id {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid loop id stack"));
                }
            }

            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Not currently in a loop"))
        }

        Ok(())
    }

    fn visit_while_statement<'a, 'b>(&mut self, context: &mut WhileStatementContext<'a, 'b>) -> io::Result<()> {
        self.loop_ids.push(context.while_statement.id);

        Ok(())
    }

    fn leave_while_statement<'a, 'b>(&mut self, context: &mut WhileStatementContext<'a, 'b>) -> io::Result<()> {
        match self.loop_ids.pop() {
            Some(loop_id) => if loop_id != context.while_statement.id {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid loop id stack"));
            }

            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Not currently in a loop"))
        }

        Ok(())
    }

    fn visit_identifier<'a, 'b>(&mut self, context: &mut IdentifierContext<'a, 'b>) -> io::Result<()> {
        match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(_) |
            ContractDefinitionNode::ModifierDefinition(_) if !self.loop_ids.is_empty() => (),
            _ => return Ok(())
        }

        for source_unit in context.source_units.iter() {
            let called_function_definition = match source_unit.function_definition(context.identifier.referenced_declaration) {
                Some(function_definition) => function_definition,
                None => continue,
            };

            if let Visibility::External = called_function_definition.visibility {
                self.print_message(
                    context.contract_definition,
                    context.definition_node,
                    context.current_source_unit.source_line(context.identifier.src.as_str())?,
                    &self.function_calls.last().unwrap().clone(),
                );
                break;
            }
        }

        Ok(())
    }

    fn visit_member_access<'a, 'b>(&mut self, context: &mut MemberAccessContext<'a, 'b>) -> io::Result<()> {
        match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(_) |
            ContractDefinitionNode::ModifierDefinition(_) if !self.loop_ids.is_empty() => (),
            _ => return Ok(())
        }

        if let Some(referenced_declaration) = context.member_access.referenced_declaration {
            for source_unit in context.source_units.iter() {
                if let Some(function_definition) =
                    source_unit.function_definition(referenced_declaration)
                {
                    if let Visibility::External = function_definition.visibility {
                        self.print_message(
                            context.contract_definition,
                            context.definition_node,
                            context.current_source_unit.source_line(context.member_access.src.as_str())?,
                            &self.function_calls.last().unwrap().clone(),
                        );
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
