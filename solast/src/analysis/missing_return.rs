use super::AstVisitor;

use solidity::ast::{
    Assignment, Block, ContractDefinition, ContractDefinitionNode,
    FunctionDefinition, NodeID, Return, SourceUnit, Statement
};

use std::{
    collections::{HashMap, HashSet},
    io,
};

struct FunctionInfo {
    assigned_return_variables: HashSet<NodeID>,
}

pub struct MissingReturnVisitor {
    function_info: HashMap<NodeID, FunctionInfo>,
}

impl Default for MissingReturnVisitor {
    fn default() -> Self {
        Self {
            function_info: HashMap::new(),
        }
    }
}

impl AstVisitor for MissingReturnVisitor {
    fn visit_function_definition(
        &mut self,
        _source_unit: &SourceUnit,
        _contract_definition: &ContractDefinition,
        _definition_node: &ContractDefinitionNode,
        function_definition: &FunctionDefinition,
    ) -> io::Result<()> {
        if function_definition.return_parameters.parameters.is_empty() {
            return Ok(())
        }

        if function_definition.body.is_none() {
            return Ok(())
        }

        if !self.function_info.contains_key(&function_definition.id) {
            self.function_info.insert(
                function_definition.id,
                FunctionInfo {
                    assigned_return_variables: HashSet::new(),
                },
            );
        }

        Ok(())
    }

    fn leave_function_definition(
        &mut self,
        _source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        _definition_node: &ContractDefinitionNode,
        function_definition: &FunctionDefinition,
    ) -> io::Result<()> {
        let function_info = match self.function_info.get(&function_definition.id) {
            Some(function_info) => function_info,
            None => return Ok(())
        };
        
        if let Some(Statement::Return(Return { .. })) = function_definition.body.as_ref().unwrap().statements.last() {
            return Ok(())
        }

        let mut assigned = vec![];

        for variable_declaration in function_definition.return_parameters.parameters.iter() {
            assigned.push(function_info.assigned_return_variables.contains(&variable_declaration.id));
        }

        if assigned.iter().all(|assigned| !assigned) {
            println!(
                "\t{} {} {} is missing an explicit return statement",

                format!("{:?}", function_definition.visibility),

                if function_definition.name.is_empty() {
                    format!("{}", contract_definition.name)
                } else {
                    format!("{}.{}", contract_definition.name, function_definition.name)
                },

                function_definition.kind
            );
        }

        Ok(())
    }

    fn visit_assignment<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        _contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        assignment: &'a Assignment,
    ) -> io::Result<()> {
        let function_definition = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(())
        };

        let function_info = match self.function_info.get_mut(&function_definition.id) {
            Some(function_info) => function_info,
            _ => return Ok(())
        };

        for id in function_definition.get_assigned_return_variables(assignment.left_hand_side.as_ref()) {
            if !function_info.assigned_return_variables.contains(&id) {
                function_info.assigned_return_variables.insert(id);
            }
        }

        Ok(())
    }
}
