use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, io, rc::Rc};

//
// TODO:
// * determine if balance can actually be manipulated
// * determine if manipulating balance has consequences
//

pub struct ManipulatableBalanceUsageVisitor {
    report: Rc<RefCell<Report>>,
}

impl ManipulatableBalanceUsageVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }

    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        expression: &dyn std::fmt::Display
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} contains manipulatable balance usage: `{}`",
                contract_definition.definition_node_location(definition_node),
                expression
            ),
        );
    }
}

impl AstVisitor for ManipulatableBalanceUsageVisitor {
    fn visit_member_access<'a, 'b>(&mut self, context: &mut MemberAccessContext<'a, 'b>) -> io::Result<()> {
        if context.member_access.member_name != "balance" {
            return Ok(())
        }
        
        let (expression, arguments) = match context.member_access.expression.as_ref() {
            Expression::FunctionCall(FunctionCall {
                expression,
                arguments,
                ..
            }) => (expression, arguments),

            _ => return Ok(())
        };
    
        match expression.as_ref() {
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_name: TypeName::ElementaryTypeName(ElementaryTypeName {
                    name: type_name,
                    ..
                }),
                ..
            }) if type_name == "address" => {}

            _ => return Ok(())
        }
    
        if arguments.len() != 1 {
            return Ok(())
        }

        match arguments.first().unwrap() {
            Expression::Identifier(Identifier {
                name,
                ..
            }) if name == "this" => {}

            _ => return Ok(())
        }

        self.add_report_entry(
            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
            context.contract_definition,
            context.definition_node,
            context.current_source_unit.source_line(context.member_access.src.as_str())?,
            context.member_access
        );

        Ok(())
    }

    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        match context.function_call.expression.as_ref() {
            Expression::MemberAccess(MemberAccess {
                member_name,
                ..
            }) if member_name == "balanceOf" => {}

            _ => return Ok(())
        }

        if context.function_call.arguments.len() != 1 {
            return Ok(())
        }

        let (expression, arguments) = match context.function_call.arguments.first().unwrap() {
            Expression::FunctionCall(FunctionCall {
                expression,
                arguments,
                ..
            }) => (expression, arguments),

            _ => return Ok(())
        };
        
        match expression.as_ref() {
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_name: TypeName::ElementaryTypeName(ElementaryTypeName {
                    name: type_name,
                    ..
                }),
                ..
            }) if type_name == "address" => {}

            _ => return Ok(())
        }
    
        if arguments.len() != 1 {
            return Ok(())
        }

        match arguments.first().unwrap() {
            Expression::Identifier(Identifier {
                name,
                ..
            }) if name == "this" => {}

            _ => return Ok(())
        }

        self.add_report_entry(
            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
            context.contract_definition,
            context.definition_node,
            context.current_source_unit.source_line(context.function_call.src.as_str())?,
            context.function_call
        );

        Ok(())
    }
}
