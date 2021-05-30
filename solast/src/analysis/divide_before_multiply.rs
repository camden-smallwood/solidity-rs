use super::AstVisitor;
use solidity::ast::{NodeID, SourceUnit};
use std::{
    collections::{HashMap, HashSet},
    io,
};

pub struct DivideBeforeMultiplyVisitor<'a> {
    pub source_units: &'a [SourceUnit],
    reported_functions: HashSet<NodeID>,
    function_variable_operations: HashMap<NodeID, HashMap<NodeID, Vec<String>>>,
}

impl<'a> DivideBeforeMultiplyVisitor<'a> {
    pub fn new(source_units: &'a [SourceUnit]) -> Self {
        Self {
            source_units,
            reported_functions: HashSet::new(),
            function_variable_operations: HashMap::new(),
        }
    }
}

impl AstVisitor for DivideBeforeMultiplyVisitor<'_> {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if !self
            .function_variable_operations
            .contains_key(&function_definition.id)
        {
            self.function_variable_operations
                .insert(function_definition.id, HashMap::new());
        }

        Ok(())
    }

    fn visit_variable_declaration<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        variable_declaration: &'a solidity::ast::VariableDeclaration,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => {
                function_definition.id
            }
            solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                modifier_definition.id
            }
            _ => return Ok(()),
        };

        let variable_operations = self
            .function_variable_operations
            .get_mut(&definition_id)
            .unwrap();

        if !variable_operations.contains_key(&variable_declaration.id) {
            variable_operations.insert(variable_declaration.id, vec![]);
        }

        Ok(())
    }

    fn visit_binary_operation<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        binary_operation: &'a solidity::ast::BinaryOperation,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => {
                function_definition.id
            }
            solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                modifier_definition.id
            }
            _ => return Ok(()),
        };

        if binary_operation.operator != "*" {
            return Ok(());
        }

        if let solidity::ast::Expression::BinaryOperation(left_operation) =
            binary_operation.left_expression.as_ref()
        {
            if left_operation.contains_operation("/") {
                if !self.reported_functions.contains(&definition_id) {
                    self.reported_functions.insert(definition_id);

                    match definition_node {
                        solidity::ast::ContractDefinitionNode::FunctionDefinition(function_definition) => {
                            println!(
                                "\t{} {} {} performs a multiplication on the result of a division",
                                format!("{:?}", function_definition.visibility),
                                if function_definition.name.is_empty() {
                                    format!("{}", contract_definition.name)
                                } else {
                                    format!("{}.{}", contract_definition.name, function_definition.name)
                                },
                                format!("{:?}", function_definition.kind).to_lowercase()
                            );
                        }

                        solidity::ast::ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                            println!(
                                "\t{} {} modifier performs a multiplication on the result of a division",
                                format!("{:?}", modifier_definition.visibility),
                                if modifier_definition.name.is_empty() {
                                    format!("{}", contract_definition.name)
                                } else {
                                    format!("{}.{}", contract_definition.name, modifier_definition.name)
                                }
                            );
                        }

                        _ => ()
                    }
                }
            }
        }

        Ok(())
    }

    fn visit_assignment<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        assignment: &'a solidity::ast::Assignment,
    ) -> io::Result<()> {
        match assignment.operator.as_str() {
            "=" => {
                // TODO: check if assignment.initial_value contains divide-before-multiply
                // TODO: check if assignment.initial_value contains divide, mark assignment.left_hand_side for multiply watch
            }

            "/=" => {
                // TODO: mark assignment.left_hand_side for multiply watch
            }

            "*=" => {
                // TODO: check if assignment.left_hand_side is marked, report divide-before-multiply
            }

            _ => {}
        }

        Ok(())
    }
}
