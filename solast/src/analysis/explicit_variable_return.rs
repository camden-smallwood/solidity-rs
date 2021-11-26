use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::{collections::HashSet, io};

#[derive(Default)]
pub struct ExplicitVariableReturnVisitor{
    local_variable_ids: HashSet<NodeID>,
}

impl ExplicitVariableReturnVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        description: &str,
        expression: &dyn std::fmt::Display
    ) {
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} returns {} explicitly: `{}`",
    
                source_line,
    
                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    format!("{}", "constructor")
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },
    
                contract_definition.name,
                contract_definition.kind,
    
                description,

                expression
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The `{}` modifier in the `{}` {} returns {} explicitly: `{}`",

                source_line,

                modifier_definition.name,

                contract_definition.name,
                contract_definition.kind,
    
                description,

                expression
            ),

            _ => {}
        }
    }
}

impl AstVisitor for ExplicitVariableReturnVisitor {
    fn visit_variable_declaration_statement<'a, 'b>(&mut self, context: &mut VariableDeclarationStatementContext<'a, 'b>) -> io::Result<()> {
        for declaration in context.variable_declaration_statement.declarations.iter() {
            if let Some(declaration) = declaration {
                if !self.local_variable_ids.contains(&declaration.id) {
                    self.local_variable_ids.insert(declaration.id);
                }
            }
        }

        Ok(())
    }

    fn visit_return<'a, 'b>(&mut self, context: &mut ReturnContext<'a, 'b>) -> io::Result<()> {
        match context.return_statement.expression.as_ref() {
            Some(Expression::Identifier(identifier)) => {
                if self.local_variable_ids.contains(&identifier.referenced_declaration) {
                    self.print_message(
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(context.return_statement.src.as_str())?,
                        "a local variable",
                        context.return_statement
                    );
                }
            }

            Some(Expression::TupleExpression(tuple_expression)) => {
                let mut all_local_variables = true;

                for component in tuple_expression.components.iter() {
                    if let Some(component) = component {
                        if let Expression::Identifier(identifier) = component {
                            if !self.local_variable_ids.contains(&identifier.referenced_declaration) {
                                all_local_variables = false;
                                break;
                            }
                        } else {
                            all_local_variables = false;
                            break;
                        }
                    }
                }

                if all_local_variables {
                    self.print_message(
                        context.contract_definition,
                        context.definition_node,
                        context.current_source_unit.source_line(context.return_statement.src.as_str())?,
                        "local variables",
                        context.return_statement
                    );
                }
            }

            _ => {}
        }

        Ok(())
    }
}
