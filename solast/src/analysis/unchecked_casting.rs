use super::AstVisitor;
use solidity::ast::*;
use std::io;

pub struct UncheckedCastingVisitor;

impl AstVisitor for UncheckedCastingVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut super::FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if context.function_call.kind != FunctionCallKind::TypeConversion {
            return Ok(())
        }

        let type_name = match context.function_call.expression.as_ref() {
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_name: TypeName::ElementaryTypeName(type_name),
                ..
            }) => type_name,

            _ => return Ok(())
        };

        if type_name.name.starts_with("int") || type_name.name.starts_with("uint") {
            //
            // TODO: if the argument is a non-literal, verify its expression was
            //       checked for validity via require, if/else or a conditional
            //
        }

        Ok(())
    }
}
