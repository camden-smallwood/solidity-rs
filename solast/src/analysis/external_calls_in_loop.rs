use super::{AstVisitor, FunctionDefinitionContext};
use solidity::ast::*;
use std::io;

pub struct ExternalCallsInLoopVisitor {
    loop_ids: Vec<NodeID>,
    makes_external_call: bool,
}

impl Default for ExternalCallsInLoopVisitor {
    fn default() -> Self {
        Self {
            loop_ids: vec![],
            makes_external_call: false,
        }
    }
}

impl AstVisitor for ExternalCallsInLoopVisitor {
    fn visit_function_definition<'a>(&mut self, _context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        self.loop_ids.clear();
        self.makes_external_call = false;

        Ok(())
    }

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if self.makes_external_call {
            println!(
                "\t{} {} {} makes an external call inside a loop",

                format!("{:?}", context.function_definition.visibility),

                if context.function_definition.name.is_empty() {
                    format!("{}", context.contract_definition.name)
                } else {
                    format!("{}.{}", context.contract_definition.name, context.function_definition.name)
                },
                
                context.function_definition.kind
            );
        }

        Ok(())
    }

    fn visit_for_statement<'a, 'b>(&mut self, context: &mut super::ForStatementContext<'a, 'b>) -> io::Result<()> {
        self.loop_ids.push(context.for_statement.id);

        Ok(())
    }

    fn leave_for_statement<'a, 'b>(&mut self, context: &mut super::ForStatementContext<'a, 'b>) -> io::Result<()> {
        match self.loop_ids.pop() {
            Some(loop_id) => {
                if loop_id != context.for_statement.id {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "asdf"));
                }
            }

            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not currently in a loop",
                ))
            }
        }

        Ok(())
    }

    fn visit_while_statement<'a, 'b>(&mut self, context: &mut super::WhileStatementContext<'a, 'b>) -> io::Result<()> {
        self.loop_ids.push(context.while_statement.id);

        Ok(())
    }

    fn leave_while_statement<'a, 'b>(&mut self, context: &mut super::WhileStatementContext<'a, 'b>) -> io::Result<()> {
        match self.loop_ids.pop() {
            Some(loop_id) => {
                if loop_id != context.while_statement.id {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "asdf"));
                }
            }

            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Not currently in a loop",
                ))
            }
        }

        Ok(())
    }

    fn visit_identifier<'a, 'b>(&mut self, context: &mut super::IdentifierContext<'a, 'b>) -> io::Result<()> {
        match context.definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(_) => {}
            _ => return Ok(())
        }

        if self.loop_ids.is_empty() || self.makes_external_call {
            return Ok(());
        }

        for source_unit in context.source_units.iter() {
            let function_definition =
                match source_unit.function_definition(context.identifier.referenced_declaration) {
                    Some(function_definition) => function_definition,
                    None => continue,
                };

            if let solidity::ast::Visibility::External = function_definition.visibility {
                self.makes_external_call = true;
                break;
            }
        }

        Ok(())
    }

    fn visit_member_access<'a, 'b>(&mut self, context: &mut super::MemberAccessContext<'a, 'b>) -> io::Result<()> {
        match context.definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(_) => {}
            _ => return Ok(())
        }

        if self.loop_ids.is_empty() || self.makes_external_call {
            return Ok(());
        }

        if let Some(referenced_declaration) = context.member_access.referenced_declaration {
            for source_unit in context.source_units.iter() {
                if let Some(function_definition) =
                    source_unit.function_definition(referenced_declaration)
                {
                    if let solidity::ast::Visibility::External = function_definition.visibility {
                        self.makes_external_call = true;
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
