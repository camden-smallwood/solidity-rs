use super::{AstVisitor, AstWalker, CallGraph};
use crate::truffle;
use solidity::ast::NodeID;
use std::{collections::{HashMap, HashSet}, io, path::PathBuf};

#[derive(Clone, Debug)]
struct FunctionState {
    is_payable: bool,
    sends_value: bool,
}

pub struct ContractLockingEtherVisitor<'a, 'b> {
    display_output: bool,
    files: &'a [truffle::File],
    call_graph: &'b CallGraph,
    contract_states: HashMap<NodeID, HashMap<NodeID, FunctionState>>,
    visited_imports: HashSet<NodeID>,
}

impl<'a, 'b> ContractLockingEtherVisitor<'a, 'b> {
    pub fn new(files: &'a [truffle::File], call_graph: &'b CallGraph) -> Self {
        Self {
            display_output: true,
            files,
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

        for file in self.files.iter() {
            let source_unit = match file.ast.as_ref() {
                Some(source_unit) => source_unit,
                None => continue,
            };

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

        for file in self.files.iter() {
            match file.ast.as_ref() {
                Some(source_unit) if source_unit.id == import_directive.source_unit => (),
                Some(source_unit) if source_unit.absolute_path == import_directive.absolute_path => (),
                
                Some(source_unit) => {
                    let import_path = PathBuf::from(import_directive.absolute_path.as_str());
                    let source_path = PathBuf::from(source_unit.absolute_path.as_str());

                    if import_path.file_stem() == source_path.file_stem() {
                        println!("Found relative path: {} {}", import_directive.absolute_path, source_unit.absolute_path);
                    } else {
                        continue;
                    }
                }

                None => {
                    println!("WARNING: file has no AST: {}", file.source_path.as_ref().unwrap().as_str());
                    continue;
                }
            }

            let mut walker = AstWalker::default();

            let mut visitor = Box::new(ContractLockingEtherVisitor::new(
                self.files,
                self.call_graph,
            ));
            visitor.display_output = false;

            walker.visitors.push(visitor);

            walker.analyze_file(file)?;

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

                    for file in self.files.iter() {
                        if let Some(contract_definition) = file.contract_definition(contract_id) {
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
