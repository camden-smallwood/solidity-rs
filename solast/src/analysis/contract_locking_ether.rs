use super::{AstVisitor, AstWalker, CallGraph};
use solidity::ast::{NodeID, SourceUnit};
use std::{collections::{HashMap, HashSet}, io};

#[derive(Clone, Debug)]
struct FunctionState {
    is_payable: bool,
    sends_value: bool,
}

pub struct ContractLockingEtherVisitor<'a, 'b> {
    display_output: bool,
    source_units: &'a [SourceUnit],
    call_graph: &'b CallGraph,
    contract_states: HashMap<NodeID, HashMap<NodeID, FunctionState>>,
    visited_imports: HashSet<NodeID>,
}

impl<'a, 'b> ContractLockingEtherVisitor<'a, 'b> {
    pub fn new(source_units: &'a [SourceUnit], call_graph: &'b CallGraph) -> Self {
        Self {
            display_output: true,
            source_units,
            call_graph,
            contract_states: HashMap::new(),
            visited_imports: HashSet::new(),
        }
    }

    fn build_function_state(
        &self,
        function_id: NodeID,
        out_state: &mut FunctionState,
        built_states: &mut HashSet<NodeID>,
    ) {
        if built_states.contains(&function_id) {
            return;
        }

        built_states.insert(function_id);

        for source_unit in self.source_units.iter() {
            let (contract_definition, function_definition) =
                match source_unit.function_and_contract_definition(function_id) {
                    Some(definitions) => definitions,
                    None => continue,
                };

            if let solidity::ast::ContractKind::Interface = contract_definition.kind {
                continue;
            }

            let function_states = match self.contract_states.get(&contract_definition.id) {
                Some(function_states) => function_states,
                None => continue,
            };

            let function_state = match function_states.get(&function_id) {
                Some(function_state) => function_state,
                None => continue,
            };

            if function_state.is_payable {
                out_state.is_payable = true;
            }

            if function_state.sends_value {
                out_state.sends_value = true;
            }

            let contract_info = self
                .call_graph
                .contracts
                .get(&contract_definition.id)
                .unwrap();
            let function_info = contract_info
                .functions
                .get(&function_definition.id)
                .unwrap();

            if function_info.sends_value {
                out_state.sends_value = true;
            }

            for call_info in function_info.calls.iter() {
                self.build_function_state(call_info.function_id, out_state, built_states);
            }

            break;
        }
    }
}

impl<'a, 'b> AstVisitor for ContractLockingEtherVisitor<'a, 'b> {
    fn visit_import_directive(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        import_directive: &solidity::ast::ImportDirective,
    ) -> io::Result<()> {
        if self.visited_imports.contains(&import_directive.source_unit) {
            return Ok(());
        }

        self.visited_imports.insert(import_directive.source_unit);

        for source_unit in self.source_units.iter() {
            let mut walker = AstWalker::default();

            let mut visitor = Box::new(ContractLockingEtherVisitor::new(
                self.source_units,
                self.call_graph,
            ));
            visitor.display_output = false;

            walker.visitors.push(visitor);

            walker.analyze_file(source_unit)?;

            let visitor = unsafe {
                &*(walker.visitors[0].as_ref() as *const _ as *const ContractLockingEtherVisitor)
            };

            for (&contract_id, function_states) in visitor.contract_states.iter() {
                if !self.contract_states.contains_key(&contract_id) {
                    self.contract_states.insert(contract_id, HashMap::new());
                }

                let states = self.contract_states.get_mut(&contract_id).unwrap();

                for (&function_id, temp_state) in function_states.iter() {
                    if !states.contains_key(&function_id) {
                        states.insert(
                            function_id,
                            FunctionState {
                                is_payable: false,
                                sends_value: false,
                            },
                        );
                    }

                    let state = states.get_mut(&function_id).unwrap();

                    if temp_state.is_payable {
                        state.is_payable = true;
                    }

                    if temp_state.sends_value {
                        state.sends_value = true;
                    }
                }
            }

            break;
        }

        Ok(())
    }

    fn visit_contract_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
    ) -> io::Result<()> {
        if !self.contract_states.contains_key(&contract_definition.id) {
            let _ = self
                .contract_states
                .insert(contract_definition.id, HashMap::new());
        }

        Ok(())
    }

    fn leave_contract_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
    ) -> io::Result<()> {
        if let solidity::ast::ContractKind::Interface = contract_definition.kind {
            return Ok(());
        }

        if contract_definition.is_abstract.unwrap_or(false) {
            return Ok(());
        }

        if !self.display_output {
            return Ok(());
        }

        let mut total_function_state = FunctionState {
            is_payable: false,
            sends_value: false,
        };

        let mut built_states = HashSet::new();

        for &contract_id in contract_definition.linearized_base_contracts.iter() {
            let function_states = match self.contract_states.get(&contract_id) {
                Some(x) => x,
                None => {
                    let mut nonloaded_contract = None;

                    for source_unit in self.source_units.iter() {
                        if let Some(contract_definition) = source_unit.contract_definition(contract_id) {
                            nonloaded_contract = Some(contract_definition);
                            break;
                        }
                    }

                    match nonloaded_contract {
                        Some(contract_definition) => println!("WARNING: contract not loaded: {}", contract_definition.name),
                        None => println!("WARNING: contract id not loaded: {}", contract_id),
                    }
                    
                    continue;
                }
            };

            for (&function_id, function_state) in function_states {
                if function_state.is_payable {
                    total_function_state.is_payable = true;
                }

                if function_state.sends_value {
                    total_function_state.sends_value = true;
                }

                self.build_function_state(
                    function_id,
                    &mut total_function_state,
                    &mut built_states,
                );
            }
        }

        if total_function_state.is_payable && !total_function_state.sends_value {
            println!(
                "\t{:?} {} has ether locking",
                contract_definition.kind, contract_definition.name
            );
        }

        Ok(())
    }

    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        _definition_node: &solidity::ast::ContractDefinitionNode,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        if let solidity::ast::FunctionKind::Constructor = function_definition.kind {
            return Ok(());
        }

        let function_states = self
            .contract_states
            .get_mut(&contract_definition.id)
            .unwrap();

        if !function_states.contains_key(&function_definition.id) {
            function_states.insert(
                function_definition.id,
                FunctionState {
                    is_payable: false,
                    sends_value: false,
                },
            );
        }

        let function_state = function_states.get_mut(&function_definition.id).unwrap();

        if let solidity::ast::StateMutability::Payable = function_definition.state_mutability {
            function_state.is_payable = true;
        }

        Ok(())
    }
}
