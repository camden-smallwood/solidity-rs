use solidity::ast::*;

pub struct ArrayAssignmentVisitor;

impl ArrayAssignmentVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        index_access: &IndexAccess,
        operator: &str,
        expression: &Expression,
    ) {
        println!(
            "\t{} contains an inefficient array assignment which can be optimized to `{} {}= {};`",
            contract_definition.definition_node_location(source_line, definition_node),
            index_access,
            operator,
            expression,
        );
    }
}

impl AstVisitor for ArrayAssignmentVisitor {
    fn visit_assignment<'a, 'b>(&mut self, context: &mut AssignmentContext<'a, 'b>) -> std::io::Result<()> {
        if context.assignment.operator != "=" {
            return Ok(());
        }

        let index_access = match context.assignment.left_hand_side.as_ref() {
            Expression::IndexAccess(index_access) => index_access,
            _ => return Ok(()),
        };

        let binary_operation = match context.assignment.right_hand_side.as_ref() {
            Expression::BinaryOperation(binary_operation) => binary_operation,
            _ => return Ok(()),
        };

        if !matches!(binary_operation.operator.as_str(), "+" | "-" | "*" | "/" | "%" | "<<" | ">>" | "&" | "|" | "^") {
            return Ok(());
        }

        let index_access2 = match binary_operation.left_expression.as_ref() {
            Expression::IndexAccess(index_access2) => index_access2,
            _ => return Ok(()),
        };

        if index_access.base_expression == index_access2.base_expression {
            self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.assignment.src.as_str())?,
                index_access,
                binary_operation.operator.as_str(),
                binary_operation.right_expression.as_ref(),
            );
        }

        Ok(())
    }
}
