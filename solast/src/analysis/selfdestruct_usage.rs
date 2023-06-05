use crate::report::Report;
use solidity::ast::*;
use std::{rc::Rc, cell::RefCell};

pub struct SelfdestructUsageVisitor {
    report: Rc<RefCell<Report>>,
}

impl SelfdestructUsageVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} contains `selfdestruct` usage",
                contract_definition.definition_node_location(definition_node),
            ),
        );
    }
}

impl AstVisitor for SelfdestructUsageVisitor {
    fn visit_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> std::io::Result<()> {
        if let Statement::ExpressionStatement(ExpressionStatement {
            expression: Expression::FunctionCall(FunctionCall {
                expression,
                src,
                ..
            })
        }) = context.statement {
            if let Expression::Identifier(Identifier { name, .. }) = expression.as_ref() {
                if name == "selfdestruct" {
                    self.add_report_entry(
                        context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(src.as_str())?,
                    );
                }
            }
        }

        Ok(())
    }
}
