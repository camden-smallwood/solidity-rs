use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, rc::Rc};

pub struct AddressBalanceVisitor {
    report: Rc<RefCell<Report>>,
}

impl AddressBalanceVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        expression: &dyn std::fmt::Display,
        external: bool,
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} contains `{}` usage, which can be optimized with assembly: `{}`",
                contract_definition.definition_node_location(definition_node),
                expression,
                if external {
                    "assembly { bal := balance(addr); }"
                } else {
                    "assembly { bal := selfbalance(); }"
                }
            ),
        );
    }
}

impl AstVisitor for AddressBalanceVisitor {
    fn visit_member_access<'a, 'b>(&mut self, context: &mut MemberAccessContext<'a, 'b>) -> std::io::Result<()> {
        if context.member_access.member_name != "balance" {
            return Ok(())
        }

        let (expression, arguments) = match context.member_access.expression.as_ref() {
            Expression::FunctionCall(FunctionCall {
                expression,
                arguments,
                ..
            }) if arguments.len() == 1 => (expression, arguments),

            _ => return Ok(())
        };
    
        match expression.as_ref() {
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_name: TypeName::ElementaryTypeName(ElementaryTypeName {
                    name,
                    ..
                }),
                ..
            }) if name == "address" => {}

            _ => return Ok(())
        }
        
        if let Some(Expression::Identifier(Identifier {
            name,
            ..
        })) = arguments.first() {
            self.add_report_entry(
                context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.member_access.src.as_str())?,
                context.member_access,
                name != "this"
            );
        }

        Ok(())
    }
}
