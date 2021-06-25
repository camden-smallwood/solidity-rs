use super::AstVisitor;
use solidity::ast::{NodeID, SourceUnit};
use std::{collections::HashMap, io};

pub struct RequireWithoutMessageVisitor<'a> {
    pub source_units: &'a [SourceUnit],
    pub requirement_counts: HashMap<NodeID, usize>,
}

impl<'a> RequireWithoutMessageVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self {
            source_units,
            requirement_counts: HashMap::new(),
        }
    }
}

impl AstVisitor for RequireWithoutMessageVisitor<'_> {
    fn leave_modifier_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        modifier_definition: &solidity::ast::ModifierDefinition,
    ) -> io::Result<()> {
        if let Some(&requirement_count) = self.requirement_counts.get(&modifier_definition.id) {
            println!(
                "\t{} {} modifier has {} without {}",

                format!("{:?}", modifier_definition.visibility),

                if modifier_definition.name.is_empty() {
                    format!("{}", contract_definition.name)
                } else {
                    format!("{}.{}", contract_definition.name, modifier_definition.name)
                },

                if requirement_count > 1 {
                    "requirements"
                } else {
                    "a requirement"
                },

                if requirement_count > 1 {
                    "messages"
                } else {
                    "a message"
                }
            );
        }

        Ok(())
    }

    fn leave_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if let Some(&requirement_count) = self.requirement_counts.get(&function_definition.id) {
            println!(
                "\t{} {} {} has {} without {}",

                format!("{:?}", function_definition.visibility),

                if function_definition.name.is_empty() {
                    format!("{}", contract_definition.name)
                } else {
                    format!("{}.{}", contract_definition.name, function_definition.name)
                },

                function_definition.kind,

                if requirement_count > 1 {
                    "requirements"
                } else {
                    "a requirement"
                },

                if requirement_count > 1 {
                    "messages"
                } else {
                    "a message"
                }
            );
        }

        Ok(())
    }

    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        function_call: &'a solidity::ast::FunctionCall,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        match function_call.expression.as_ref() {
            solidity::ast::Expression::Identifier(expr) if expr.name == "require" => (),
            _ => return Ok(())
        }

        if function_call.arguments.len() < 2 {
            if !self.requirement_counts.contains_key(&definition_id) {
                self.requirement_counts.insert(definition_id, 0);
            }

            *self.requirement_counts.get_mut(&definition_id).unwrap() += 1;
        }

        Ok(())
    }
}
