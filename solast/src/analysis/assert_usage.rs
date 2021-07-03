use super::AstVisitor;
use solidity::ast::*;
use std::{collections::HashSet, io};

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
    fn visit_function_call<'a, 'b>(&mut self, context: &mut super::FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if let Expression::Identifier(Identifier { name, .. }) = context.function_call.expression.as_ref() {
            if name == "assert" {
                match context.definition_node {
                    ContractDefinitionNode::FunctionDefinition(function_definition) => {
                        if !self.reported_definitions.contains(&function_definition.id) {
                            self.reported_definitions.insert(function_definition.id);

                            println!(
                                "\t{} {} {} contains `assert` usage",
                                format!("{:?}", function_definition.visibility),
                                if function_definition.kind == FunctionKind::Constructor {
                                    format!("{}", context.contract_definition.name)
                                } else {
                                    format!("{}.{}", context.contract_definition.name, function_definition.name)
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
                                format!("{}.{}", context.contract_definition.name, modifier_definition.name)
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
