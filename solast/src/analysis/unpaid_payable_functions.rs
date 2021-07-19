use solidity::ast::*;
use std::io;

pub struct UnpaidPayableFunctionsVisitor;

impl UnpaidPayableFunctionsVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        called_contract_definition: &ContractDefinition,
        called_definition_node: &ContractDefinitionNode,
    ) {
        println!(
            "\t{} {} {} makes a call to the {} payable {} {} without paying",

            format!(
                "{:?}",
                match definition_node {
                    ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition.visibility,
                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => modifier_definition.visibility,
                    _ => unimplemented!("{:?}", definition_node),
                }
            ),

            {
                let name = format!(
                    "{}",
                    match definition_node {
                        ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition.name.as_str(),
                        ContractDefinitionNode::ModifierDefinition(modifier_definition) => modifier_definition.name.as_str(),
                        _ => unimplemented!("{:?}", definition_node),
                    }
                );

                if name.is_empty() {
                    format!("{}", contract_definition.name)
                } else {
                    format!("{}.{}", contract_definition.name, name)
                }
            },

            match definition_node {
                ContractDefinitionNode::FunctionDefinition(function_definition) => format!("{}", function_definition.kind),
                ContractDefinitionNode::ModifierDefinition(_) => "modifier".into(),
                _ => unimplemented!("{:?}", definition_node),
            },

            format!(
                "{:?}",
                match called_definition_node {
                    ContractDefinitionNode::FunctionDefinition(called_function_definition) => called_function_definition.visibility,
                    ContractDefinitionNode::ModifierDefinition(called_modifier_definition) => called_modifier_definition.visibility,
                    _ => unimplemented!("{:?}", called_definition_node),
                }
            ).to_lowercase(),

            {
                let name = format!(
                    "{}",
                    match called_definition_node {
                        ContractDefinitionNode::FunctionDefinition(called_function_definition) => called_function_definition.name.as_str(),
                        ContractDefinitionNode::ModifierDefinition(called_modifier_definition) => called_modifier_definition.name.as_str(),
                        _ => unimplemented!("{:?}", called_definition_node),
                    }
                );

                if name.is_empty() {
                    format!("{}", called_contract_definition.name)
                } else {
                    format!("{}.{}", called_contract_definition.name, name)
                }
            },

            match called_definition_node {
                ContractDefinitionNode::FunctionDefinition(called_function_definition) => format!("{}", called_function_definition.kind),
                ContractDefinitionNode::ModifierDefinition(_) => "modifier".into(),
                _ => unimplemented!("{:?}", called_definition_node),
            }
        );
    }
}

impl AstVisitor for UnpaidPayableFunctionsVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        match context.function_call.expression.as_ref() {
            solidity::ast::Expression::Identifier(identifier) => {
                for source_unit in context.source_units.iter() {
                    if let Some((called_contract_definition, called_definition_node)) = source_unit.find_contract_definition_node(identifier.referenced_declaration) {
                        if let ContractDefinitionNode::FunctionDefinition(FunctionDefinition {
                            state_mutability: StateMutability::Payable,
                            ..
                        }) = called_definition_node {
                            self.print_message(
                                context.contract_definition,
                                context.definition_node,
                                called_contract_definition,
                                called_definition_node,
                            );
                        }
                        break;
                    }
                }
            }

            solidity::ast::Expression::MemberAccess(member_access) => {
                let referenced_declaration = match member_access.referenced_declaration {
                    Some(id) => id,
                    None => return Ok(()),
                };

                for source_unit in context.source_units.iter() {
                    if let Some((called_contract_definition, called_definition_node)) = source_unit.find_contract_definition_node(referenced_declaration) {
                        if let ContractDefinitionNode::FunctionDefinition(FunctionDefinition {
                            state_mutability: StateMutability::Payable,
                            ..
                        }) = called_definition_node {
                            self.print_message(
                                context.contract_definition,
                                context.definition_node,
                                called_contract_definition,
                                called_definition_node,
                            );
                        }
                        break;
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }
}
