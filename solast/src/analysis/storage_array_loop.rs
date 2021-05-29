use super::AstVisitor;
use crate::truffle;
use solidity::ast::NodeID;
use std::{
    collections::{HashMap, HashSet},
    io,
};

struct FunctionInfo {
    pub loops_over_storage_array: bool,
}

pub struct StorageArrayLoopVisitor<'a> {
    pub files: &'a [truffle::File],
    storage_arrays: HashSet<NodeID>,
    functions: HashMap<NodeID, FunctionInfo>,
}

impl<'a> StorageArrayLoopVisitor<'a> {
    pub fn new(files: &'a [truffle::File]) -> Self {
        Self {
            files,
            storage_arrays: HashSet::new(),
            functions: HashMap::new(),
        }
    }

    fn expression_contains_storage_array_length(&self, expression: &solidity::ast::Expression) -> bool {
        match expression {
            solidity::ast::Expression::BinaryOperation(binary_operation) => {
                if self.expression_contains_storage_array_length(
                    binary_operation.left_expression.as_ref(),
                ) {
                    return true;
                }

                self.expression_contains_storage_array_length(
                    binary_operation.right_expression.as_ref(),
                )
            }

            solidity::ast::Expression::Conditional(conditional) => {
                if self
                    .expression_contains_storage_array_length(conditional.true_expression.as_ref())
                {
                    return true;
                }

                self.expression_contains_storage_array_length(conditional.false_expression.as_ref())
            }

            solidity::ast::Expression::Assignment(assignment) => {
                self.expression_contains_storage_array_length(assignment.right_hand_side.as_ref())
            }

            solidity::ast::Expression::MemberAccess(member_access) => {
                let referenced_declarations = member_access.expression.referenced_declarations();

                if referenced_declarations.len() == 0 {
                    return false;
                }

                member_access.member_name == "length"
                    && self
                        .storage_arrays
                        .contains(referenced_declarations.iter().last().unwrap_or(&0))
            }

            solidity::ast::Expression::TupleExpression(tuple_expression) => {
                for component in tuple_expression.components.iter() {
                    if let Some(component) = component {
                        if self.expression_contains_storage_array_length(component) {
                            return true;
                        }
                    }
                }

                false
            }

            _ => false,
        }
    }
}

impl AstVisitor for StorageArrayLoopVisitor<'_> {
    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        _contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if !self.functions.contains_key(&function_definition.id) {
            self.functions.insert(
                function_definition.id,
                FunctionInfo {
                    loops_over_storage_array: false,
                },
            );
        }

        for variable_declaration in function_definition.parameters.parameters.iter() {
            if let solidity::ast::StorageLocation::Storage = variable_declaration.storage_location {
                if let Some(solidity::ast::TypeName::ArrayTypeName(_)) = variable_declaration.type_name {
                    if !self.storage_arrays.contains(&variable_declaration.id) {
                        self.storage_arrays.insert(variable_declaration.id);
                    }
                }
            }
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
        if let Some(function_info) = self.functions.get(&function_definition.id) {
            if function_info.loops_over_storage_array {
                println!(
                    "\t{} {} {} performs a loop over a storage array, querying the length over each iteration",

                    format!("{:?}", function_definition.visibility),

                    if function_definition.name.is_empty() {
                        format!("{}", contract_definition.name)
                    } else {
                        format!("{}.{}", contract_definition.name, function_definition.name)
                    },

                    format!("{:?}", function_definition.kind).to_lowercase()
                );
            }
        }

        Ok(())
    }

    fn visit_variable_declaration<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        variable_declaration: &'a solidity::ast::VariableDeclaration,
    ) -> io::Result<()> {
        let storage_location = match &variable_declaration.storage_location {
            solidity::ast::StorageLocation::Default if variable_declaration.state_variable => {
                solidity::ast::StorageLocation::Storage
            }
            storage_location => storage_location.clone(),
        };

        if let solidity::ast::StorageLocation::Storage = storage_location {
            if let Some(solidity::ast::TypeName::ArrayTypeName(_)) = variable_declaration.type_name {
                if !self.storage_arrays.contains(&variable_declaration.id) {
                    self.storage_arrays.insert(variable_declaration.id);
                }
            }
        }

        Ok(())
    }

    fn visit_for_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        for_statement: &'a solidity::ast::ForStatement,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        if let Some(expression) = for_statement.condition.as_ref() {
            if self.expression_contains_storage_array_length(expression) {
                self.functions
                    .get_mut(&definition_id)
                    .unwrap()
                    .loops_over_storage_array = true;
            }
        }

        Ok(())
    }

    fn visit_while_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        while_statement: &solidity::ast::WhileStatement,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        if self.expression_contains_storage_array_length(&while_statement.condition) {
            self.functions
                .get_mut(&definition_id)
                .unwrap()
                .loops_over_storage_array = true;
        }

        Ok(())
    }
}
