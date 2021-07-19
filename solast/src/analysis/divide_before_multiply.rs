use solidity::ast::*;
use std::io;

pub struct DivideBeforeMultiplyVisitor;

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
                match context.definition_node {
                    ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                        "\tL{}: The {} {} in the `{}` {} performs a multiplication on the result of a division",
    
                        context.current_source_unit.source_line(context.binary_operation.src.as_str()).unwrap(),

                        function_definition.visibility,
    
                        if let FunctionKind::Constructor = function_definition.kind {
                            format!("{}", function_definition.kind)
                        } else {
                            format!("`{}` {}", function_definition.name, function_definition.kind)
                        },
    
                        context.contract_definition.name,
                        context.contract_definition.kind
                    ),
    
                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                        "\tL{}: The `{}` modifier in the `{}` {} performs a multiplication on the result of a division",

                        context.current_source_unit.source_line(context.binary_operation.src.as_str()).unwrap(),

                        modifier_definition.name,
    
                        context.contract_definition.name,
                        context.contract_definition.kind
                    ),
    
                    _ => ()
                }
            }
        }

        Ok(())
    }
}
