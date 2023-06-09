use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

pub struct IneffectualStatementsVisitor {
    report: Rc<RefCell<Report>>,
}

impl IneffectualStatementsVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        description: &str,
        expression: &dyn std::fmt::Display
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} contains an ineffectual {} statement: `{}`",
                contract_definition.definition_node_location(definition_node),
                description,
                expression
            ),
        );
    }
}

impl AstVisitor for IneffectualStatementsVisitor {
    fn visit_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> io::Result<()> {
        let expression = match context.statement {
            Statement::ExpressionStatement(ExpressionStatement { expression }) => expression,
            _ => return Ok(())
        };

        let (source_line, description, expression): (usize, &str, &dyn std::fmt::Display) = match expression {
            Expression::Literal(literal) => (
                context.current_source_unit.source_line(literal.src.as_str())?,
                "literal",
                literal
            ),
            Expression::Identifier(identifier) => (
                context.current_source_unit.source_line(identifier.src.as_str())?,
                "identifier",
                identifier
            ),
            Expression::IndexAccess(index_access) => (
                context.current_source_unit.source_line(index_access.src.as_str())?,
                "index access",
                index_access
            ),
            Expression::IndexRangeAccess(index_range_access) => (
                context.current_source_unit.source_line(index_range_access.src.as_str())?,
                "index range access",
                index_range_access
            ),
            Expression::MemberAccess(member_access) => (
                context.current_source_unit.source_line(member_access.src.as_str())?,
                "member access",
                member_access
            ),
            Expression::BinaryOperation(binary_operation) => (
                context.current_source_unit.source_line(binary_operation.src.as_str())?,
                "binary operation",
                binary_operation
            ),
            Expression::Conditional(conditional) => (
                context.current_source_unit.source_line(conditional.src.as_str())?,
                "conditional",
                conditional
            ),
            Expression::TupleExpression(tuple_expression) => (
                context.current_source_unit.source_line(tuple_expression.src.as_str())?,
                "tuple expression",
                tuple_expression
            ),
            Expression::FunctionCallOptions(function_call_options) => (
                context.current_source_unit.source_line(function_call_options.src.as_str())?,
                "function call options",
                function_call_options
            ),
            _ => return Ok(()),
        };
        
        self.add_report_entry(
            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
            context.contract_definition,
            context.definition_node,
            source_line,
            description,
            expression,
        );
        
        Ok(())
    }
}
