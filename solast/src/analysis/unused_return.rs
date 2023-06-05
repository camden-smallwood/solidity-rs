use crate::report::Report;
use solidity::ast::*;
use std::{cell::RefCell, rc::Rc};

pub struct UnusedReturnVisitor {
    report: Rc<RefCell<Report>>,
}

impl UnusedReturnVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self { report }
    }
}

impl AstVisitor for UnusedReturnVisitor {
    fn visit_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> std::io::Result<()> {
        let (referenced_declaration, src) = match context.statement {
            Statement::ExpressionStatement(ExpressionStatement {
                expression: Expression::FunctionCall(FunctionCall {
                    arguments,
                    expression,
                    src,
                    ..
                })
            }) if !arguments.is_empty() => match expression.root_expression() {
                Some(&Expression::Identifier(Identifier {
                    referenced_declaration,
                    ..
                })) => (referenced_declaration, src),

                Some(&Expression::MemberAccess(MemberAccess {
                    referenced_declaration: Some(referenced_delcaration),
                    ..
                })) => (referenced_delcaration, src),

                _ => return Ok(())
            }

            _ => return Ok(())
        };

        for source_unit in context.source_units.iter() {
            if let Some((called_contract_definition, called_function_definition)) = source_unit.function_and_contract_definition(referenced_declaration) {
                if !called_function_definition.return_parameters.parameters.is_empty() {
                    self.report.borrow_mut().add_entry(
                        context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                        Some(context.current_source_unit.source_line(src)?),
                        format!(
                            "{} makes a call to the {}, ignoring the returned {}",

                            context.contract_definition.definition_node_location(context.definition_node),
                            
                            format!(
                                "{} `{}` {}",
    
                                format!("{:?}", called_function_definition.visibility).to_lowercase(),
        
                                if called_function_definition.name.is_empty() {
                                    called_contract_definition.name.to_string()
                                } else {
                                    format!("{}.{}", called_contract_definition.name, called_function_definition.name)
                                },
        
                                format!("{:?}", called_function_definition.kind).to_lowercase(),
                            ),
                            
                            if called_function_definition.return_parameters.parameters.len() == 1 { "value" } else { "values" },
                        ),
                    );
                }
                
                break;
            }
        }

        Ok(())
    }
}
