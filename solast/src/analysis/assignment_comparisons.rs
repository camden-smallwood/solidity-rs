use solidity::ast::*;
use std::io;

pub struct AssignmentComparisonsVisitor;

impl AstVisitor for AssignmentComparisonsVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        let called_function_name = match context.function_call.expression.as_ref() {
            Expression::Identifier(Identifier { name, .. }) if name == "require" || name == "assert" => name,
            _ => return Ok(())
        };

        if context.function_call.arguments.first().unwrap().contains_operation("=") {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tL{}: The {} {} in the `{}` {} contains a call to `{}` that performs an assignment: `{}`",

                    context.current_source_unit.source_line(context.function_call.src.as_str()).unwrap(),

                    function_definition.visibility,

                    if let FunctionKind::Constructor = function_definition.kind {
                        format!("{}", function_definition.kind)
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    called_function_name,

                    context.function_call
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tL{}: The `{}` modifier in the `{}` {} contains a call to `{}` that performs an assignment: `{}`",

                    context.current_source_unit.source_line(context.function_call.src.as_str()).unwrap(),

                    modifier_definition.name,

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    called_function_name,

                    context.function_call
                ),

                _ => ()
            }
        }

        Ok(())
    }

    fn visit_if_statement<'a, 'b>(&mut self, context: &mut IfStatementContext<'a, 'b>) -> io::Result<()> {
        if context.if_statement.condition.contains_operation("=") {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tL{}: The {} {} in the `{}` {} contains an if statement with a condition that performs an assignment: `{}`",
                    
                    context.current_source_unit.source_line(context.if_statement.src.as_str()).unwrap(),

                    function_definition.visibility,

                    if let FunctionKind::Constructor = function_definition.kind {
                        format!("{}", function_definition.kind)
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    context.if_statement
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tL{}: The `{}` modifier in the `{}` {} contains an if statement with a condition that performs an assignment: `{}`",
                    
                    context.current_source_unit.source_line(context.if_statement.src.as_str()).unwrap(),

                    modifier_definition.name,

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    context.if_statement
                ),

                _ => ()
            }
        }

        Ok(())
    }

    fn visit_for_statement<'a, 'b>(&mut self, context: &mut ForStatementContext<'a, 'b>) -> io::Result<()> {
        if let Some(condition) = context.for_statement.condition.as_ref() {
            if condition.contains_operation("=") {
                match context.definition_node {
                    ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                        "\tL{}: The {} {} in the `{}` {} contains a for statement with a condition that performs an assignment: `{}`",
    
                        context.current_source_unit.source_line(context.for_statement.src.as_str()).unwrap(),

                        function_definition.visibility,
    
                        if let FunctionKind::Constructor = function_definition.kind {
                            format!("{}", function_definition.kind)
                        } else {
                            format!("`{}` {}", function_definition.name, function_definition.kind)
                        },
    
                        context.contract_definition.name,
                        context.contract_definition.kind,

                        context.for_statement
                    ),
    
                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                        "\tL{}: The `{}` modifier in the `{}` {} contains a for statement with a condition that performs an assignment: `{}`",
                        
                        context.current_source_unit.source_line(context.for_statement.src.as_str()).unwrap(),

                        modifier_definition.name,
    
                        context.contract_definition.name,
                        context.contract_definition.kind,

                        context.for_statement
                    ),
    
                    _ => ()
                }
            }
        }

        Ok(())
    }

    fn visit_while_statement<'a, 'b>(&mut self, context: &mut WhileStatementContext<'a, 'b>) -> io::Result<()> {
        if context.while_statement.condition.contains_operation("=") {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tL{}: The {} {} in the `{}` {} contains a while statement with a condition that performs an assignment: `{}`",
                    
                    context.current_source_unit.source_line(context.while_statement.src.as_str()).unwrap(),

                    function_definition.visibility,

                    if let FunctionKind::Constructor = function_definition.kind {
                        format!("{}", function_definition.kind)
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    context.while_statement
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tL{}: The `{}` modifier in the `{}` {} contains a while statement with a condition that performs an assignment: `{}`",

                    context.current_source_unit.source_line(context.while_statement.src.as_str()).unwrap(),

                    modifier_definition.name,

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    context.while_statement
                ),

                _ => ()
            }
        }

        Ok(())
    }

    fn visit_conditional<'a, 'b>(&mut self, context: &mut ConditionalContext<'a, 'b>) -> io::Result<()> {
        if context.conditional.condition.contains_operation("=") {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tL{}: The {} {} in the `{}` {} contains a conditional expression with a condition that performs an assignment: `{}`",

                    context.current_source_unit.source_line(context.conditional.src.as_str()).unwrap(),

                    function_definition.visibility,

                    if let FunctionKind::Constructor = function_definition.kind {
                        format!("{}", function_definition.kind)
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    context.conditional
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tL{}: The `{}` modifier in the `{}` {} contains a conditional expression with a condition that performs an assignment: `{}`",

                    context.current_source_unit.source_line(context.conditional.src.as_str()).unwrap(),

                    modifier_definition.name,

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    context.conditional
                ),

                _ => ()
            }
        }

        Ok(())
    }
}
