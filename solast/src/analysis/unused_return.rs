use solidity::ast::*;

pub struct UnusedReturnVisitor;

impl UnusedReturnVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        called_name: &str,
        return_parameter_count: usize,
    ) {
        println!(
            "\t{} makes a call to the {}, ignoring the returned {}",
            contract_definition.definition_node_location(source_line, definition_node),
            called_name,
            if return_parameter_count == 1 { "value" } else { "values" },
        );
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
                    self.print_message(
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(src)?,

                        format!(
                            "{} `{}` {}",

                            format!("{:?}", called_function_definition.visibility).to_lowercase(),
    
                            if called_function_definition.name.is_empty() {
                                called_contract_definition.name.to_string()
                            } else {
                                format!("{}.{}", called_contract_definition.name, called_function_definition.name)
                            },
    
                            format!("{:?}", called_function_definition.kind).to_lowercase(),
                        ).as_str(),

                        called_function_definition.return_parameters.parameters.len(),
                    );
                }
                
                break;
            }
        }

        Ok(())
    }
}
