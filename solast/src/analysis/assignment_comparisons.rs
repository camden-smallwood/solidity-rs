use super::AstVisitor;
use solidity::ast::*;
use std::io;

pub struct AssignmentComparisonsVisitor;

impl AstVisitor for AssignmentComparisonsVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut super::FunctionCallContext<'a, 'b>) -> io::Result<()> {
        let called_function_name = match context.function_call.expression.as_ref() {
            Expression::Identifier(Identifier { name, .. }) if name == "require" || name == "assert" => name,
            _ => return Ok(())
        };

        if context.function_call.arguments.first().unwrap().contains_operation("=") {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tThe {} {} in the `{}` {} contains a call to `{}` that performs an assignment",

                    function_definition.visibility,

                    if let FunctionKind::Constructor = function_definition.kind {
                        format!("{}", function_definition.kind)
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    called_function_name
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tThe `{}` modifier in the `{}` {} contains a call to `{}` that performs an assignment",
                    modifier_definition.name,

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    called_function_name
                ),

                _ => ()
            }
        }

        Ok(())
    }

    fn visit_if_statement<'a, 'b>(&mut self, context: &mut super::IfStatementContext<'a, 'b>) -> io::Result<()> {
        if context.if_statement.condition.contains_operation("=") {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tThe {} {} in the `{}` {} contains an if statement with a condition that performs an assignment",

                    function_definition.visibility,

                    if let FunctionKind::Constructor = function_definition.kind {
                        format!("{}", function_definition.kind)
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },

                    context.contract_definition.name,
                    context.contract_definition.kind
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tThe `{}` modifier in the `{}` {} contains an if statement with a condition that performs an assignment",
                    modifier_definition.name,

                    context.contract_definition.name,
                    context.contract_definition.kind
                ),

                _ => ()
            }
        }

        Ok(())
    }

    fn visit_for_statement<'a, 'b>(&mut self, context: &mut super::ForStatementContext<'a, 'b>) -> io::Result<()> {
        if let Some(condition) = context.for_statement.condition.as_ref() {
            if condition.contains_operation("=") {
                match context.definition_node {
                    ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                        "\tThe {} {} in the `{}` {} contains a for statement with a condition that performs an assignment",
    
                        function_definition.visibility,
    
                        if let FunctionKind::Constructor = function_definition.kind {
                            format!("{}", function_definition.kind)
                        } else {
                            format!("`{}` {}", function_definition.name, function_definition.kind)
                        },
    
                        context.contract_definition.name,
                        context.contract_definition.kind
                    ),
    
                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                        "\tThe `{}` modifier in the `{}` {} contains a for statement with a condition that performs an assignment",
                        modifier_definition.name,
    
                        context.contract_definition.name,
                        context.contract_definition.kind
                    ),
    
                    _ => ()
                }
            }
        }

        Ok(())
    }

    fn visit_while_statement<'a, 'b>(&mut self, context: &mut super::WhileStatementContext<'a, 'b>) -> io::Result<()> {
        if context.while_statement.condition.contains_operation("=") {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tThe {} {} in the `{}` {} contains a while statement with a condition that performs an assignment",

                    function_definition.visibility,

                    if let FunctionKind::Constructor = function_definition.kind {
                        format!("{}", function_definition.kind)
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },

                    context.contract_definition.name,
                    context.contract_definition.kind
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tThe `{}` modifier in the `{}` {} contains a while statement with a condition that performs an assignment",
                    modifier_definition.name,

                    context.contract_definition.name,
                    context.contract_definition.kind
                ),

                _ => ()
            }
        }

        Ok(())
    }

    fn visit_conditional<'a, 'b>(&mut self, context: &mut super::ConditionalContext<'a, 'b>) -> io::Result<()> {
        if context.conditional.condition.contains_operation("=") {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tThe {} {} in the `{}` {} contains a conditional expression with a condition that performs an assignment",

                    function_definition.visibility,

                    if let FunctionKind::Constructor = function_definition.kind {
                        format!("{}", function_definition.kind)
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },

                    context.contract_definition.name,
                    context.contract_definition.kind
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tThe `{}` modifier in the `{}` {} contains a conditional expression with a condition that performs an assignment",
                    modifier_definition.name,

                    context.contract_definition.name,
                    context.contract_definition.kind
                ),

                _ => ()
            }
        }

        Ok(())
    }
}
