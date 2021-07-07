use super::{AstVisitor, FunctionCallContext};

pub struct AbiEncodingVisitor;

impl AstVisitor for AbiEncodingVisitor {
    fn visit_function_call<'a, 'b>(&mut self, _context: &mut FunctionCallContext<'a, 'b>) -> std::io::Result<()> {
        //
        // TODO: check if multiple VLAs are encoded together
        // if so, print a message about potential hash collisions
        //

        Ok(())
    }
}