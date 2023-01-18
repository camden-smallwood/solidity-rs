use solidity::ast::*;
use std::io;

pub struct IneffectualStatementsVisitor;

impl IneffectualStatementsVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        description: &str,
        expression: &dyn std::fmt::Display
    ) {
        println!(
            "\t{} contains an ineffectual {} statement: `{}`",
            contract_definition.definition_node_location(source_line, definition_node),
            description,
            expression
        );
    }
}

impl AstVisitor for IneffectualStatementsVisitor {
    fn visit_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> io::Result<()> {
        let expression = match context.statement {
            Statement::ExpressionStatement(ExpressionStatement { expression }) => expression,
            _ => return Ok(())
        };
    
        match expression {
            Expression::Literal(literal) => self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(literal.src.as_str())?,
                "literal",
                literal
            ),
            
            Expression::Identifier(identifier) => self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(identifier.src.as_str())?,
                "identifier",
                identifier
            ),

            Expression::IndexAccess(index_access) => self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(index_access.src.as_str())?,
                "index access",
                index_access
            ),

            Expression::IndexRangeAccess(index_range_access) => self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(index_range_access.src.as_str())?,
                "index range access",
                index_range_access
            ),

            Expression::MemberAccess(member_access) => self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(member_access.src.as_str())?,
                "member access",
                member_access
            ),

            Expression::BinaryOperation(binary_operation) => self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(binary_operation.src.as_str())?,
                "binary operation",
                binary_operation
            ),

            Expression::Conditional(conditional) => self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(conditional.src.as_str())?,
                "conditional",
                conditional
            ),

            Expression::TupleExpression(tuple_expression) => self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(tuple_expression.src.as_str())?,
                "tuple expression",
                tuple_expression
            ),
            
            Expression::FunctionCallOptions(function_call_options) => self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(function_call_options.src.as_str())?,
                "function call options",
                function_call_options
            ),

            _ => {}
        }

        Ok(())
    }
}
