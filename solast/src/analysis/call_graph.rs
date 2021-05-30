use super::AstVisitor;
use solidity::ast::{
    Block, ContractDefinition, ContractDefinitionNode, Expression, FunctionCall, FunctionCallKind,
    FunctionCallOptions, FunctionDefinition, Identifier, MemberAccess, NodeID, SourceUnit,
    Statement, Visibility,
};
use std::{collections::HashMap, io};

#[derive(Clone, Debug)]
pub struct CallInfo {
    pub function_id: NodeID,
    pub arguments: Vec<Option<NodeID>>,
}

impl CallInfo {
    pub fn makes_external_call(&self, source_units: &[SourceUnit]) -> bool {
        for source_unit in source_units.iter() {
            if let Some(function_definition) = source_unit.function_definition(self.function_id) {
                if let Visibility::External = function_definition.visibility {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Clone, Debug)]
pub struct FunctionInfo {
    pub id: NodeID,
    pub calls: Vec<CallInfo>,
    pub is_payable: bool,
    pub sends_value: bool,
}

impl FunctionInfo {
    pub fn makes_external_call(&self, source_units: &[SourceUnit]) -> bool {
        for call_info in self.calls.iter() {
            if call_info.makes_external_call(source_units) {
                return true;
            }
        }

        false
    }
}

#[derive(Clone, Debug)]
pub struct ContractInfo {
    pub id: NodeID,
    pub functions: HashMap<NodeID, FunctionInfo>,
}

#[derive(Clone, Debug)]
pub struct CallGraph {
    pub contracts: HashMap<NodeID, ContractInfo>,
    current_call_arguments: Option<Vec<Option<NodeID>>>,
}

impl Default for CallGraph {
    fn default() -> Self {
        Self {
            contracts: HashMap::new(),
            current_call_arguments: None,
        }
    }
}

impl CallGraph {
    pub fn build(source_units: &[SourceUnit]) -> io::Result<Self> {
        let mut analyzer = super::AstWalker::default();

        analyzer.visitors.push(Box::new(Self::default()));
        analyzer.analyze(source_units)?;

        Ok(unsafe {
            (&*(analyzer.visitors.pop().unwrap().as_ref() as *const _ as *const Self)).clone()
        })
    }

    pub fn contract_info(&self, id: NodeID) -> Option<&ContractInfo> {
        self.contracts.get(&id)
    }

    pub fn function_info(&self, id: NodeID) -> Option<&FunctionInfo> {
        for (_, contract_info) in self.contracts.iter() {
            if let Some(function_info) = contract_info.functions.get(&id) {
                return Some(function_info);
            }
        }

        None
    }

    pub fn function_and_contract_info(
        &self,
        function_id: NodeID,
    ) -> Option<(&ContractInfo, &FunctionInfo)> {
        for (_, contract_info) in self.contracts.iter() {
            if let Some(function_info) = contract_info.functions.get(&function_id) {
                return Some((contract_info, function_info));
            }
        }

        None
    }

    pub fn hierarchy_contains_state_variable(
        &self,
        source_units: &[SourceUnit],
        contract_definition: &ContractDefinition,
        state_variable_id: NodeID,
    ) -> bool {
        // Loop through all of the contracts in the supplied contract's inheritance hierarchy
        for &contract_id in contract_definition.linearized_base_contracts.iter() {
            // Loop through all of the schema source_units in the project
            for source_unit in source_units.iter() {
                // Attempt to retrieve the current contract in the inheritance hierarchy from the current schema source_unit
                let contract_definition = match source_unit.contract_definition(contract_id) {
                    Some(contract_definition) => contract_definition,
                    None => continue,
                };

                // Attempt to retrieve the requested state variable from the current contract in the inheritance hierarchy
                if let Some(_) = contract_definition.variable_declaration(state_variable_id) {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_assigned_state_variables(
        &self,
        source_units: &[SourceUnit],
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        expression: &Expression,
    ) -> io::Result<Vec<NodeID>> {
        let mut ids = vec![];

        match expression {
            Expression::Identifier(identifier) => {
                if self.hierarchy_contains_state_variable(
                    source_units,
                    contract_definition,
                    identifier.referenced_declaration,
                ) {
                    ids.push(identifier.referenced_declaration);
                }
            }

            Expression::Assignment(assignment) => {
                ids.extend(self.get_assigned_state_variables(
                    source_units,
                    contract_definition,
                    definition_node,
                    assignment.left_hand_side.as_ref(),
                )?);
            }

            Expression::IndexAccess(index_access) => {
                ids.extend(self.get_assigned_state_variables(
                    source_units,
                    contract_definition,
                    definition_node,
                    index_access.base_expression.as_ref(),
                )?);
            }

            Expression::IndexRangeAccess(index_range_access) => {
                ids.extend(self.get_assigned_state_variables(
                    source_units,
                    contract_definition,
                    definition_node,
                    index_range_access.base_expression.as_ref(),
                )?);
            }

            Expression::MemberAccess(member_access) => {
                ids.extend(self.get_assigned_state_variables(
                    source_units,
                    contract_definition,
                    definition_node,
                    member_access.expression.as_ref(),
                )?);
            }

            Expression::TupleExpression(tuple_expression) => {
                for component in tuple_expression.components.iter() {
                    if let Some(component) = component {
                        ids.extend(self.get_assigned_state_variables(
                            source_units,
                            contract_definition,
                            definition_node,
                            component,
                        )?);
                    }
                }
            }

            _ => (),
        }

        Ok(ids)
    }
}

impl AstVisitor for CallGraph {
    fn visit_contract_definition(
        &mut self,
        _source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
    ) -> io::Result<()> {
        if !self.contracts.contains_key(&contract_definition.id) {
            self.contracts.insert(
                contract_definition.id,
                ContractInfo {
                    id: contract_definition.id,
                    functions: HashMap::new(),
                },
            );
        }

        Ok(())
    }

    fn visit_function_definition(
        &mut self,
        _source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        _definition_node: &ContractDefinitionNode,
        function_definition: &FunctionDefinition,
    ) -> io::Result<()> {
        let contract_info = self.contracts.get_mut(&contract_definition.id).unwrap();

        if !contract_info.functions.contains_key(&function_definition.id) {
            contract_info.functions.insert(function_definition.id, FunctionInfo {
                id: function_definition.id,
                calls: vec![],
                is_payable: function_definition.state_mutability == solidity::ast::StateMutability::Payable,
                sends_value: false,
            });
        }

        Ok(())
    }

    fn visit_identifier(
        &mut self,
        _source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        _blocks: &mut Vec<&Block>,
        _statement: Option<&Statement>,
        identifier: &Identifier,
    ) -> io::Result<()> {
        let definition_id = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                function_definition.id
            }
            ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                modifier_definition.id
            }
            _ => return Ok(()),
        };

        let contract_info = self.contracts.get_mut(&contract_definition.id).unwrap();
        let function_info = match contract_info.functions.get_mut(&definition_id) {
            Some(function_info) => function_info,
            None => return Ok(()),
        };

        if let Some(arguments) = self.current_call_arguments.as_ref() {
            if identifier.argument_types.is_some()
                && identifier.referenced_declaration <= i32::MAX as _
            {
                function_info.calls.push(CallInfo {
                    function_id: identifier.referenced_declaration,
                    arguments: arguments.clone(),
                });
                self.current_call_arguments = None;
            }
        }

        Ok(())
    }

    fn visit_function_call(
        &mut self,
        _source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        _blocks: &mut Vec<&Block>,
        _statement: Option<&Statement>,
        function_call: &FunctionCall,
    ) -> io::Result<()> {
        if let FunctionCallKind::TypeConversion = function_call.kind {
            return Ok(());
        }

        let function_definition = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(()),
        };

        let contract_info = self.contracts.get_mut(&contract_definition.id).unwrap();
        let function_info = contract_info
            .functions
            .get_mut(&function_definition.id)
            .unwrap();

        if function_definition.name == "transfer" {
            if let Expression::MemberAccess(member_access) = function_call.expression.as_ref() {
                if member_access.referenced_declaration.is_none() && member_access.member_name == "sender" {
                    function_info.sends_value = true;
                }
            }
        }

        let mut arguments = vec![];

        for argument in function_call.arguments.iter() {
            arguments.push(
                if let Expression::Identifier(Identifier {
                    referenced_declaration,
                    ..
                }) = argument
                {
                    if let Some(parameter) = function_definition
                        .parameters
                        .parameters
                        .iter()
                        .find(|p| p.id.eq(referenced_declaration))
                    {
                        Some(parameter.id)
                    } else {
                        None
                    }
                } else {
                    None
                },
            );
        }

        self.current_call_arguments = Some(arguments);

        Ok(())
    }

    fn visit_function_call_options(
        &mut self,
        _source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        _blocks: &mut Vec<&Block>,
        _statement: Option<&Statement>,
        function_call_options: &FunctionCallOptions,
    ) -> io::Result<()> {
        let function_definition = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(()),
        };

        let contract_info = self.contracts.get_mut(&contract_definition.id).unwrap();
        let function_info = contract_info
            .functions
            .get_mut(&function_definition.id)
            .unwrap();

        if self.current_call_arguments.is_some() && function_call_options.names.iter().any(|name| name == "value") {
            function_info.sends_value = true;
        }

        Ok(())
    }

    fn visit_member_access(
        &mut self,
        _source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        _blocks: &mut Vec<&Block>,
        _statement: Option<&Statement>,
        member_access: &MemberAccess,
    ) -> io::Result<()> {
        let function_definition = match definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(()),
        };

        let contract_info = self.contracts.get_mut(&contract_definition.id).unwrap();
        let function_info = contract_info
            .functions
            .get_mut(&function_definition.id)
            .unwrap();

        if let Some(arguments) = self.current_call_arguments.as_ref() {
            if let Some(referenced_declaration) = member_access.referenced_declaration {
                if member_access.argument_types.is_some() && referenced_declaration <= i32::MAX as _
                {
                    function_info.calls.push(CallInfo {
                        function_id: referenced_declaration,
                        arguments: arguments.clone(),
                    });

                    self.current_call_arguments = None;
                }
            }
        }

        Ok(())
    }
}
