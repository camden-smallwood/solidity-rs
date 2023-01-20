use solidity::ast::*;
use std::io;

pub struct DivideBeforeMultiplyVisitor;

impl DivideBeforeMultiplyVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
    ) {
        println!(
            "\t{} performs a multiplication on the result of a division",
            contract_definition.definition_node_location(source_line, definition_node),
        );
    }
}

//
// TODO:
//   1. track variable assignments, transfering all operations that occurred
//   2. retrieve operations from function calls
//

impl AstVisitor for DivideBeforeMultiplyVisitor {
    fn visit_binary_operation<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> {
        if context.binary_operation.operator != "*" {
            return Ok(())
        }

        if let Expression::BinaryOperation(left_operation) = context.binary_operation.left_expression.as_ref() {
            if left_operation.contains_operation("/") {
                self.print_message(
                    context.contract_definition,
                    context.definition_node,
                    context.current_source_unit.source_line(context.binary_operation.src.as_str())?,
                );
            }
        }

        Ok(())
    }
}
