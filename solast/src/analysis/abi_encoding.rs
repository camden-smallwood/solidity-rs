use super::{AstVisitor, FunctionCallContext};
use solidity::ast::*;

pub struct AbiEncodingVisitor;

impl AstVisitor for AbiEncodingVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> std::io::Result<()> {
        //
        // Only check for calls to abi.encodePacked(...)
        //

        if let Expression::MemberAccess(MemberAccess { expression, member_name, .. }) = context.function_call.expression.as_ref() {
            if let Expression::Identifier(Identifier { name, .. }) = expression.as_ref() {
                if name != "abi" || member_name != "encodePacked" {
                    return Ok(())
                }
            } else {
                return Ok(())
            }
        } else {
            return Ok(())
        }

        //
        // Only check if multiple arguments are supplied: abi.encodePacked(as, bs, ...)
        //

        if context.function_call.arguments.len() <= 1 {
            return Ok(())
        }

        //
        // TODO: determine if any parameters are variably-sized arrays
        // if so, print a message warning about potential hash collisions
        //

        Ok(())
    }
}
