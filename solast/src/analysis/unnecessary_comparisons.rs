use super::{AstVisitor, BinaryOperationContext};
use primitive_types::U512;
use solidity::ast::*;
use std::{io, str::FromStr};

pub struct UnnecessaryComparisonsVisitor;

impl UnnecessaryComparisonsVisitor {
    fn handle_left_expression_comparison<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> {
        let type_name = match context.binary_operation.left_expression.type_descriptions() {
            Some(TypeDescriptions { type_string: Some(type_string), .. }) => type_string.as_str(),
            _ => return Ok(())
        };

        if !type_name.starts_with("uint") {
            return Ok(()) // TODO: handle int
        }

        let value = match context.binary_operation.right_expression.as_ref() {
            Expression::Literal(
                Literal { hex_value: Some(value), .. }
            ) => match U512::from_str(value) {
                Ok(value) => value,
                Err(_) => return Ok(())
            }

            Expression::Literal(
                Literal { value: Some(value), .. }
            ) => match U512::from_dec_str(value) {
                Ok(value) => value,
                Err(_) => return Ok(())
            }

            _ => return Ok(())
        };
        
        let mut contains_redundant_comparison = false;

        match type_name {
            type_name if type_name.starts_with("uint") => {
                let type_bits: usize = match type_name.trim_start_matches("uint") {
                    "" => 256,
                    string => match string.parse() {
                        Ok(type_bits) => type_bits,
                        Err(_) => return Ok(())
                    }
                };

                let type_max = (U512::one() << type_bits) - U512::one();

                match context.binary_operation.operator.as_str() {
                    ">=" if value.is_zero() => {
                        contains_redundant_comparison = true;
                    }

                    "<" if value >= type_max => {
                        contains_redundant_comparison = true;
                    }

                    "<=" if value > type_max => {
                        contains_redundant_comparison = true;
                    }

                    _ => {}
                }
            }

            type_name if type_name.starts_with("int") => {
                // TODO
            }

            _ => {}
        }
        
        if contains_redundant_comparison {
            match context.definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => println!(
                    "\tThe {} {} in the `{}` {} contains a redundant comparison: {}",

                    function_definition.visibility,

                    if let FunctionKind::Constructor = function_definition.kind {
                        format!("constructor")
                    } else {
                        format!("`{}` {}", function_definition.name, function_definition.kind)
                    },

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    context.binary_operation
                ),

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => println!(
                    "\tThe {} modifier in the `{}` {} contains a redundant comparison: {}",

                    modifier_definition.name,

                    context.contract_definition.name,
                    context.contract_definition.kind,

                    context.binary_operation
                ),

                _ => ()
            }
        }

        Ok(())
    }

    fn handle_right_expression_comparison<'a, 'b>(&mut self, _context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> {
        // TODO
        Ok(())
    }
}

impl AstVisitor for UnnecessaryComparisonsVisitor {
    fn visit_binary_operation<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> {
        match (context.binary_operation.left_expression.as_ref(), context.binary_operation.right_expression.as_ref()) {
            (Expression::Literal(_), Expression::Literal(_)) => Ok(()),
            (_, Expression::Literal(_)) => self.handle_left_expression_comparison(context),
            (Expression::Literal(_), _) => self.handle_right_expression_comparison(context),
            _ => Ok(())
        }
    }
}
