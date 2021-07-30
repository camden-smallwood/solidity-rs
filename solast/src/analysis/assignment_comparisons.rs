use solidity::ast::*;
use std::io;

pub struct AssignmentComparisonsVisitor;

impl AssignmentComparisonsVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        message: String,
        expression: &dyn std::fmt::Display
    ) {
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} contains a {} that performs an assignment: `{}`",

                source_line,

                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    format!("{}", function_definition.kind)
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },

                contract_definition.name,
                contract_definition.kind,

                message,

                expression
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The `{}` modifier in the `{}` {} contains a {} that performs an assignment: `{}`",

                source_line,

                modifier_definition.name,

                contract_definition.name,
                contract_definition.kind,

                message,

                expression
            ),

            _ => ()
        }
    }
}

impl AstVisitor for AssignmentComparisonsVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        let called_function_name = match context.function_call.expression.as_ref() {
            Expression::Identifier(Identifier { name, .. }) if name == "require" || name == "assert" => name,
            _ => return Ok(())
        };

        if context.function_call.arguments.first().unwrap().contains_operation("=") {
            self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.function_call.src.as_str())?,
                format!("a call to `{}`", called_function_name),
                context.function_call
            );
        }

        Ok(())
    }

    fn visit_if_statement<'a, 'b>(&mut self, context: &mut IfStatementContext<'a, 'b>) -> io::Result<()> {
        if context.if_statement.condition.contains_operation("=") {
            self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.if_statement.src.as_str())?,
                "an if statement".to_string(),
                &context.if_statement.condition
            );
        }

        Ok(())
    }

    fn visit_for_statement<'a, 'b>(&mut self, context: &mut ForStatementContext<'a, 'b>) -> io::Result<()> {
        if let Some(condition) = context.for_statement.condition.as_ref() {
            if condition.contains_operation("=") {
                self.print_message(
                    context.contract_definition,
                    context.definition_node,
                    context.current_source_unit.source_line(context.for_statement.src.as_str())?,
                    "a for statement".to_string(),
                    condition
                );
            }
        }

        Ok(())
    }

    fn visit_while_statement<'a, 'b>(&mut self, context: &mut WhileStatementContext<'a, 'b>) -> io::Result<()> {
        if context.while_statement.condition.contains_operation("=") {
            self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.while_statement.src.as_str())?,
                "a while statement".to_string(),
                &context.while_statement.condition
            );
        }

        Ok(())
    }

    fn visit_conditional<'a, 'b>(&mut self, context: &mut ConditionalContext<'a, 'b>) -> io::Result<()> {
        if context.conditional.condition.contains_operation("=") {
            self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.conditional.src.as_str())?,
                "a conditional expression".to_string(),
                context.conditional
            );
        }

        Ok(())
    }
}
