use solidity::ast::*;

pub struct SelfdestructUsageVisitor;

impl SelfdestructUsageVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
    ) {
        println!(
            "\t{} contains `selfdestruct` usage",
            contract_definition.definition_node_location(source_line, definition_node),
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
                    self.print_message(
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
