use solidity::ast::*;

pub struct AbstractContractsVisitor;

impl AbstractContractsVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        function_definition: &FunctionDefinition,
        source_line: usize,
    ) {
        println!(
            "\t{} is marked {} instead of marking `{}` as abstract",
            contract_definition.definition_node_location(source_line, definition_node),
            function_definition.visibility,
            contract_definition.name,
        );
    }
}

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
            self.print_message(
                context.contract_definition,
                context.definition_node,
                context.function_definition,
                context.current_source_unit.source_line(context.function_definition.src.as_str())?,
            );
        }

        Ok(())
    }
}
