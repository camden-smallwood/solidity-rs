use super::AstVisitor;
use solidity::ast::{
    Block, ContractDefinition, ContractDefinitionNode, Expression, FunctionCall, Identifier,
    SourceUnit, Statement,
};
use std::io;

pub struct AssignmentComparisonsVisitor;

impl AstVisitor for AssignmentComparisonsVisitor {
    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        function_call: &'a FunctionCall,
    ) -> io::Result<()> {
        if let Expression::Identifier(Identifier { name, .. }) = function_call.expression.as_ref() {
            if (name == "require" || name == "assert")
                && function_call
                    .arguments
                    .first()
                    .unwrap()
                    .contains_operation("=")
            {
                match definition_node {
                    solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => {
                        println!(
                            "\t{} {} {} contains a call to {} that performs an assignment",
                            format!("{:?}", function_definition.visibility),
                            if function_definition.name.is_empty() {
                                format!("{}", contract_definition.name)
                            } else {
                                format!("{}.{}", contract_definition.name, function_definition.name)
                            },
                            format!("{:?}", function_definition.kind).to_lowercase(),
                            name
                        );
                    }

                    solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                        println!(
                            "\t{} {} modifier contains a call to {} that performs an assignment",
                            format!("{:?}", modifier_definition.visibility),
                            if modifier_definition.name.is_empty() {
                                format!("{}", contract_definition.name)
                            } else {
                                format!("{}.{}", contract_definition.name, modifier_definition.name)
                            },
                            name
                        );
                    }

                    _ => ()
                }
            }
        }

        Ok(())
    }
}
