use solidity::ast::*;

pub struct AbstractContractsVisitor;

impl AstVisitor for AbstractContractsVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> std::io::Result<()> {
        //
        // Only check function definitions associated with constructors
        //

        if context.function_definition.kind != FunctionKind::Constructor {
            return Ok(())
        }

        //
        // Only check function definitions with internal visibility
        //

        if context.function_definition.visibility != Visibility::Internal {
            return Ok(())
        }

        //
        // If the constructor is marked internal and the contract is not abstract, print a message
        //

        if let None | Some(false) = context.contract_definition.is_abstract {
            println!(
                "\tL{}: The constructor of the `{}` contract is marked {} instead of marking `{}` as abstract",

                context.current_source_unit.source_line(context.contract_definition.src.as_str()).unwrap(),

                context.contract_definition.name,
                context.function_definition.visibility,
                context.contract_definition.name,
            );
        }

        Ok(())
    }
}
