use super::AstVisitor;
use solidity::ast::*;

pub struct SelfdestructUsageVisitor;

impl AstVisitor for SelfdestructUsageVisitor {
    fn visit_statement<'a, 'b>(&mut self, context: &mut super::StatementContext<'a, 'b>) -> std::io::Result<()> {
        if let Statement::ExpressionStatement(ExpressionStatement {
            expression: Expression::FunctionCall(FunctionCall {
                expression,
                src,
                ..
            })
        }) = context.statement {
            if let Expression::Identifier(Identifier { name, .. }) = expression.as_ref() {
                if name == "selfdestruct" {
                    match context.definition_node {
                        ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                            "\tL{}: The {} {} in the `{}` {} contains `selfdestruct` usage",
            
                            context.current_source_unit.source_line(src.as_str()).unwrap(),

                            function_definition.visibility,
            
                            if let FunctionKind::Constructor = function_definition.kind {
                                format!("{}", "constructor")
                            } else {
                                format!("`{}` {}", function_definition.name, function_definition.kind)
                            },
            
                            context.contract_definition.name,
                            context.contract_definition.kind
                        ),
            
                        ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                            "\tL{}: The {} `{}` modifier in the `{}` {} contains `selfdestruct` usage",
            
                            context.current_source_unit.source_line(src.as_str()).unwrap(),

                            modifier_definition.visibility,
                            modifier_definition.name,
            
                            context.contract_definition.name,
                            context.contract_definition.kind
                        ),
            
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
