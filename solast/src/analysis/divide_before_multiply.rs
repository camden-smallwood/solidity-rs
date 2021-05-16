use super::AstVisitor;
use crate::truffle;
use solidity::ast::NodeID;
use std::{
    collections::{HashMap, HashSet},
    io,
};

pub struct DivideBeforeMultiplyVisitor<'a> {
    pub files: &'a [truffle::File],
    reported_functions: HashSet<NodeID>,
    function_variable_operations: HashMap<NodeID, HashMap<NodeID, Vec<String>>>,
}

impl<'a> DivideBeforeMultiplyVisitor<'a> {
    pub fn new(files: &'a [truffle::File]) -> Self {
        Self {
            files,
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
        function_definition: Option<&'a solidity::ast::FunctionDefinition>,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        variable_declaration: &'a solidity::ast::VariableDeclaration,
    ) -> io::Result<()> {
        if let Some(function_definition) = function_definition {
            let variable_operations = self
                .function_variable_operations
                .get_mut(&function_definition.id)
                .unwrap();

            if !variable_operations.contains_key(&variable_declaration.id) {
                variable_operations.insert(variable_declaration.id, vec![]);
            }
        }

        Ok(())
    }

    fn visit_binary_operation<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
        function_definition: Option<&'a solidity::ast::FunctionDefinition>,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        binary_operation: &'a solidity::ast::BinaryOperation,
    ) -> io::Result<()> {
        let function_definition = match function_definition {
            Some(function_definition) => function_definition,
            None => return Ok(()),
        };

        if binary_operation.operator != "*" {
            return Ok(());
        }

        if let solidity::ast::Expression::BinaryOperation(left_operation) =
            binary_operation.left_expression.as_ref()
        {
            if left_operation.contains_operation("/") {
                if !self.reported_functions.contains(&function_definition.id) {
                    self.reported_functions.insert(function_definition.id);

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
            }
        }

        Ok(())
    }

    fn visit_assignment<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _function_definition: Option<&'a solidity::ast::FunctionDefinition>,
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
