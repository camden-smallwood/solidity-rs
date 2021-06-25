use super::AstVisitor;
use std::{collections::HashSet, io};

use solidity::ast::{Block, ContractDefinition, ContractDefinitionNode, Expression, FunctionCall, FunctionKind, Identifier, NodeID, SourceUnit, Statement};

pub struct AssertUsageVisitor {
    reported_definitions: HashSet<NodeID>,
}

impl Default for AssertUsageVisitor {
    fn default() -> Self {
        Self {
            reported_definitions: HashSet::new(),
        }
    }
}

impl AstVisitor for AssertUsageVisitor {
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
            if name == "assert" {
                match definition_node {
                    ContractDefinitionNode::FunctionDefinition(function_definition) => {
                        if !self.reported_definitions.contains(&function_definition.id) {
                            self.reported_definitions.insert(function_definition.id);

                            println!(
                                "\t{} {} {} contains `assert` usage",
                                format!("{:?}", function_definition.visibility),
                                if function_definition.kind == FunctionKind::Constructor {
                                    format!("{}", contract_definition.name)
                                } else {
                                    format!("{}.{}", contract_definition.name, function_definition.name)
                                },
                                function_definition.kind
                            );
                        }
                    }

                    ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                        if !self.reported_definitions.contains(&modifier_definition.id) {
                            self.reported_definitions.insert(modifier_definition.id);
                            
                            println!(
                                "\t{} {} modifier contains `assert` usage",
                                format!("{:?}", modifier_definition.visibility),
                                format!("{}.{}", contract_definition.name, modifier_definition.name)
                            );
                        }
                    }

                    _ => ()
                }
            }
        }
        
        Ok(())
    }
}
