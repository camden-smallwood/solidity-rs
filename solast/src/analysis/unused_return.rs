use solidity::ast::*;

pub struct UnusedReturnVisitor;

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
                if called_function_definition.return_parameters.parameters.is_empty() {
                    return Ok(())
                }
                
                match context.definition_node {
                    ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                        "\tL{}: The {} `{}` {} makes a call to the {} `{}` {}, ignoring the returned {}",
                        
                        context.current_source_unit.source_line(src).unwrap(),

                        function_definition.visibility,

                        if function_definition.name.is_empty() {
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, function_definition.name)
                        },

                        function_definition.kind,

                        format!("{:?}", called_function_definition.visibility).to_lowercase(),

                        if called_function_definition.name.is_empty() {
                            format!("{}", called_contract_definition.name)
                        } else {
                            format!("{}.{}", called_contract_definition.name, called_function_definition.name)
                        },

                        format!("{:?}", called_function_definition.kind).to_lowercase(),

                        if called_function_definition.return_parameters.parameters.len() > 1 {
                            "values"
                        } else {
                            "value"
                        }
                    ),

                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                        "\tL{}: The {} `{}` modifier makes a call to the {} `{}` {}, ignoring the returned {}",

                        context.current_source_unit.source_line(src).unwrap(),

                        format!("{:?}", modifier_definition.visibility).to_lowercase(),

                        if modifier_definition.name.is_empty() {
                            format!("{}", context.contract_definition.name)
                        } else {
                            format!("{}.{}", context.contract_definition.name, modifier_definition.name)
                        },

                        format!("{:?}", called_function_definition.visibility).to_lowercase(),

                        if called_function_definition.name.is_empty() {
                            format!("{}", called_contract_definition.name)
                        } else {
                            format!("{}.{}", called_contract_definition.name, called_function_definition.name)
                        },

                        format!("{:?}", called_function_definition.kind).to_lowercase(),

                        if called_function_definition.return_parameters.parameters.len() > 1 {
                            "values"
                        } else {
                            "value"
                        }
                    ),

                    _ => ()
                }

                return Ok(())
            }
        }

        Ok(())
    }
}
