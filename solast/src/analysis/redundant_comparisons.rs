use primitive_types::U512;
use solidity::ast::*;
use std::{io, str::FromStr};

pub struct RedundantComparisonsVisitor;

impl RedundantComparisonsVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        binary_operation: &BinaryOperation
    ) {
        match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                "\tL{}: The {} {} in the `{}` {} contains a redundant comparison: `{}`",
    
                source_line,
    
                function_definition.visibility,

                if let FunctionKind::Constructor = function_definition.kind {
                    "constructor".to_string()
                } else {
                    format!("`{}` {}", function_definition.name, function_definition.kind)
                },
    
                contract_definition.name,
                contract_definition.kind,
    
                binary_operation
            ),

            ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                "\tL{}: The `{}` modifier in the `{}` {} contains a redundant comparison: `{}`",

                source_line,

                modifier_definition.name,

                contract_definition.name,
                contract_definition.kind,
    
                binary_operation
            ),

            _ => {}
        }
    }

    fn is_right_literal_redundant<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> bool {
        let type_name = match context.binary_operation.left_expression.type_descriptions() {
            Some(TypeDescriptions { type_string: Some(type_string), .. }) => type_string.as_str(),
            _ => return false
        };

        let literal_value = match context.binary_operation.right_expression.as_ref() {
            Expression::Literal(
                Literal { hex_value: Some(value), .. }
            ) => match U512::from_str(value) {
                Ok(value) => value,
                Err(_) => return false
            }

            Expression::Literal(
                Literal { value: Some(value), .. }
            ) => match U512::from_dec_str(value) {
                Ok(value) => value,
                Err(_) => return false
            }

            _ => return false
        };
        
        let mut contains_redundant_comparison = false;

        match type_name {
            type_name if type_name.starts_with("uint") => {
                let type_bits: usize = match type_name.trim_start_matches("uint") {
                    "" => 256,
                    string => match string.parse() {
                        Ok(type_bits) => type_bits,
                        Err(_) => return false
                    }
                };

                let type_max = (U512::one() << type_bits) - U512::one();

                contains_redundant_comparison = match context.binary_operation.operator.as_str() {
                    ">=" => literal_value.is_zero(),
                    "<=" => literal_value > type_max,
                    ">" | "<" => literal_value >= type_max,
                    _ => false
                }
            }

            type_name if type_name.starts_with("int") => {
                // TODO
            }

            _ => {}
        }

        contains_redundant_comparison
    }

    fn is_left_literal_redundant<'a, 'b>(&mut self, _context: &mut BinaryOperationContext<'a, 'b>) -> bool {
        // TODO
        false
    }
}

impl AstVisitor for RedundantComparisonsVisitor {
    fn visit_binary_operation<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> {
        match context.binary_operation.operator.as_str() {
            "==" | "!=" | ">" | ">=" | "<" | "<=" => (),
            _ => return Ok(())
        }

        if match (context.binary_operation.left_expression.as_ref(), context.binary_operation.right_expression.as_ref()) {
            (Expression::Literal(_), Expression::Literal(_)) => true,
            (_, Expression::Literal(_)) => self.is_right_literal_redundant(context),
            (Expression::Literal(_), _) => self.is_left_literal_redundant(context),
            _ => false
        } {
            self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.binary_operation.src.as_str())?,
                context.binary_operation
            );
        }

        Ok(())
    }
}
