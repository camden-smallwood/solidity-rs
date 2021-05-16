use super::{AstVisitor, AstWalker, CallGraph};
use crate::truffle;
use solidity::ast::NodeID;
use std::{collections::HashSet, io};

#[derive(Clone, Debug)]
struct FunctionState {
    id: NodeID,
    name: String,
    parameters_verified: Vec<(NodeID, String, bool)>,
}

#[derive(Clone, Debug)]
struct ContractState {
    id: NodeID,
    name: String,
    function_states: Vec<FunctionState>,
}

pub struct ZeroAddressParametersVisitor<'a, 'b> {
    files: &'a [truffle::File],
    call_graph: &'b CallGraph,
    contract_states: Vec<ContractState>,
    visited_imports: HashSet<NodeID>,
    display_output: bool,
}

impl<'a, 'b> ZeroAddressParametersVisitor<'a, 'b> {
    pub fn new(files: &'a [truffle::File], call_graph: &'b CallGraph) -> Self {
        Self {
            files,
            call_graph,
            contract_states: vec![],
            visited_imports: HashSet::new(),
            display_output: true,
        }
    }

    fn build_function_state(&mut self, function_id: NodeID, built_states: &mut HashSet<NodeID>) {
        if built_states.contains(&function_id) {
            return;
        }

        built_states.insert(function_id);

        let (calling_contract_definition, calling_function_definition) = {
            let mut calling_contract_definition = None;
            let mut calling_function_definition = None;

            for file in self.files.iter() {
                let source_unit = match file.ast.as_ref() {
                    Some(source_unit) => source_unit,
                    None => continue,
                };

                match source_unit.function_and_contract_definition(function_id) {
                    Some((contract_definition, function_definition)) => {
                        calling_contract_definition = Some(contract_definition);
                        calling_function_definition = Some(function_definition);
                        break;
                    }

                    None => continue,
                }
            }

            if calling_contract_definition.is_none() || calling_function_definition.is_none() {
                return;
            }

            (
                calling_contract_definition.unwrap(),
                calling_function_definition.unwrap(),
            )
        };

        let calling_function_info = self
            .call_graph
            .function_info(calling_function_definition.id)
            .unwrap();

        for call_info in calling_function_info.calls.iter() {
            self.build_function_state(call_info.function_id, built_states);

            let (called_contract_definition, called_function_definition) = {
                let mut called_contract_definition = None;
                let mut called_function_definition = None;

                for file in self.files.iter() {
                    let source_unit = match file.ast.as_ref() {
                        Some(source_unit) => source_unit,
                        None => continue,
                    };

                    match source_unit.function_and_contract_definition(call_info.function_id) {
                        Some((contract_definition, function_definition)) => {
                            called_contract_definition = Some(contract_definition);
                            called_function_definition = Some(function_definition);
                            break;
                        }

                        None => continue,
                    }
                }

                if called_contract_definition.is_none() || called_function_definition.is_none() {
                    continue;
                }

                (
                    called_contract_definition.unwrap(),
                    called_function_definition.unwrap(),
                )
            };

            for (called_parameter_index, calling_parameter_id) in
                call_info.arguments.iter().enumerate()
            {
                if let Some(calling_parameter_id) = calling_parameter_id {
                    for (parameter_index, parameter) in calling_function_definition
                        .parameters
                        .parameters
                        .iter()
                        .enumerate()
                    {
                        if !parameter.id.eq(calling_parameter_id) {
                            continue;
                        }

                        let parameter_verified = {
                            let called_contract_state =
                                match self.contract_states.iter().find(|contract_state| {
                                    contract_state.id == called_contract_definition.id
                                }) {
                                    Some(contract_state) => contract_state,
                                    None => continue,
                                };

                            let called_function_state = match called_contract_state
                                .function_states
                                .iter()
                                .find(|function_state| {
                                    function_state.id == called_function_definition.id
                                }) {
                                Some(function_state) => function_state,
                                None => continue,
                            };

                            called_function_state.parameters_verified[called_parameter_index].2
                        };

                        if parameter_verified {
                            let calling_contract_state = self
                                .contract_states
                                .iter_mut()
                                .find(|contract_state| {
                                    contract_state.id == calling_contract_definition.id
                                })
                                .unwrap();

                            let calling_function_state = calling_contract_state
                                .function_states
                                .iter_mut()
                                .find(|function_state| {
                                    function_state.id == calling_function_definition.id
                                })
                                .unwrap();

                            calling_function_state.parameters_verified[parameter_index].2 = true;

                            break;
                        }
                    }
                }
            }
        }
    }
}

impl AstVisitor for ZeroAddressParametersVisitor<'_, '_> {
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
                _ => continue,
            };

            let mut walker = AstWalker::default();

            let mut visitor = Box::new(ZeroAddressParametersVisitor::new(
                self.files,
                self.call_graph,
            ));
            visitor.display_output = false;

            walker.visitors.push(visitor);

            walker.analyze_file(file)?;

            let visitor = unsafe {
                &*(walker.visitors[0].as_ref() as *const _ as *const ZeroAddressParametersVisitor)
            };

            for temp_contract_state in visitor.contract_states.iter() {
                let contract_id = temp_contract_state.id;

                if self
                    .contract_states
                    .iter()
                    .find(|contract_state| contract_state.id == contract_id)
                    .is_none()
                {
                    self.contract_states.push(ContractState {
                        id: contract_id,
                        name: temp_contract_state.name.clone(),
                        function_states: vec![],
                    });
                }

                let contract_state = self
                    .contract_states
                    .iter_mut()
                    .find(|contract_state| contract_state.id == contract_id)
                    .unwrap();

                for temp_function_state in temp_contract_state.function_states.iter() {
                    let function_id = temp_function_state.id;

                    if contract_state
                        .function_states
                        .iter()
                        .find(|function_state| function_state.id == function_id)
                        .is_none()
                    {
                        contract_state.function_states.push(FunctionState {
                            id: function_id,
                            name: temp_function_state.name.clone(),
                            parameters_verified: vec![],
                        });

                        let function_state = contract_state
                            .function_states
                            .iter_mut()
                            .find(|function_state| function_state.id == function_id)
                            .unwrap();

                        function_state
                            .parameters_verified
                            .extend_from_slice(temp_function_state.parameters_verified.as_slice());
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
        if self
            .contract_states
            .iter()
            .find(|contract_state| contract_state.id == contract_definition.id)
            .is_none()
        {
            self.contract_states.push(ContractState {
                id: contract_definition.id,
                name: contract_definition.name.clone(),
                function_states: vec![],
            });
        }

        return Ok(());
    }

    fn leave_contract_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
    ) -> io::Result<()> {
        let mut built_states = HashSet::new();

        let contract_state = self
            .contract_states
            .iter()
            .find(|contract_state| contract_state.id == contract_definition.id)
            .unwrap()
            .clone();

        for function_state in contract_state.function_states.iter() {
            let function_id = function_state.id;

            self.build_function_state(function_id, &mut built_states);

            let function_definition = contract_definition
                .function_definition(function_id)
                .unwrap();

            if function_definition.body.is_none() {
                return Ok(());
            }

            if self.display_output {
                let contract_state = self
                    .contract_states
                    .iter_mut()
                    .find(|contract_state| contract_state.id == contract_definition.id)
                    .unwrap();

                let function_state = contract_state
                    .function_states
                    .iter_mut()
                    .find(|function_state| function_state.id == function_definition.id)
                    .unwrap();

                for (parameter_index, parameter) in
                    function_definition.parameters.parameters.iter().enumerate()
                {
                    if parameter.name.is_empty() {
                        continue;
                    }

                    match parameter.type_name.as_ref() {
                        Some(solidity::ast::TypeName::ElementaryTypeName(
                            solidity::ast::ElementaryTypeName { name, .. },
                        ))
                        | Some(solidity::ast::TypeName::String(name))
                            if name == "address" =>
                        {
                            if !function_state.parameters_verified[parameter_index].2 {
                                println!(
                                    "\t{} {} {} does not verify if address '{}' is non-zero",
                                    format!("{:?}", function_definition.visibility),
                                    if function_definition.name.is_empty() {
                                        format!("{}", contract_definition.name)
                                    } else {
                                        format!(
                                            "{}.{}",
                                            contract_definition.name, function_definition.name
                                        )
                                    },
                                    format!("{:?}", function_definition.kind).to_lowercase(),
                                    parameter.name
                                );
                            }
                        }

                        _ => (),
                    }
                }
            }
        }

        Ok(())
    }

    fn visit_function_definition(
        &mut self,
        _source_unit: &solidity::ast::SourceUnit,
        contract_definition: &solidity::ast::ContractDefinition,
        function_definition: &solidity::ast::FunctionDefinition,
    ) -> io::Result<()> {
        let contract_state = self
            .contract_states
            .iter_mut()
            .find(|contract_state| contract_state.id == contract_definition.id)
            .unwrap();

        if contract_state
            .function_states
            .iter()
            .find(|function_state| function_state.id == function_definition.id)
            .is_none()
        {
            contract_state.function_states.push(FunctionState {
                id: function_definition.id,
                name: function_definition.name.clone(),
                parameters_verified: vec![],
            });

            let function_state = contract_state
                .function_states
                .iter_mut()
                .find(|function_state| function_state.id == function_definition.id)
                .unwrap();

            for parameter in function_definition.parameters.parameters.iter() {
                function_state.parameters_verified.push((
                    parameter.id,
                    parameter.name.clone(),
                    false,
                ));
            }
        }

        Ok(())
    }

    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        contract_definition: &'a solidity::ast::ContractDefinition,
        function_definition: Option<&'a solidity::ast::FunctionDefinition>,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _statement: Option<&'a solidity::ast::Statement>,
        function_call: &'a solidity::ast::FunctionCall,
    ) -> io::Result<()> {
        let function_definition = match function_definition {
            Some(function_definition) => function_definition,
            None => return Ok(()),
        };

        let contract_state = self
            .contract_states
            .iter_mut()
            .find(|contract_state| contract_state.id == contract_definition.id)
            .unwrap();

        let function_state = contract_state
            .function_states
            .iter_mut()
            .find(|function_state| function_state.id == function_definition.id)
            .unwrap();

        //
        // TODO: check for conditional equality/inequality to address(0) or address(0x0)
        //

        match function_call.expression.as_ref() {
            solidity::ast::Expression::Identifier(expr) if expr.name == "require" => (),
            _ => return Ok(()),
        }

        let mut operations = match function_call.arguments.first().unwrap() {
            solidity::ast::Expression::BinaryOperation(expr) => vec![expr],
            _ => return Ok(()),
        };

        while let Some(operation) = operations.pop() {
            match operation.operator.as_str() {
                "&&" | "||" => {
                    if let solidity::ast::Expression::BinaryOperation(operation) =
                        operation.left_expression.as_ref()
                    {
                        operations.push(operation);
                    }

                    if let solidity::ast::Expression::BinaryOperation(operation) =
                        operation.right_expression.as_ref()
                    {
                        operations.push(operation);
                    }
                }

                "!=" => {
                    let parameter_identifier = match operation.left_expression.as_ref() {
                        solidity::ast::Expression::Identifier(expr) => expr,
                        _ => continue,
                    };

                    let type_conversion = match operation.right_expression.as_ref() {
                        solidity::ast::Expression::FunctionCall(expr)
                            if expr.arguments.len() == 1
                                && expr.kind == solidity::ast::FunctionCallKind::TypeConversion =>
                        {
                            expr
                        }
                        _ => continue,
                    };

                    let type_name_expression = match type_conversion.expression.as_ref() {
                        solidity::ast::Expression::ElementaryTypeNameExpression(expr) => expr,
                        _ => continue,
                    };

                    match &type_name_expression.type_name {
                        solidity::ast::TypeName::ElementaryTypeName(
                            solidity::ast::ElementaryTypeName { name, .. },
                        )
                        | solidity::ast::TypeName::String(name)
                            if name == "address" =>
                        {
                            ()
                        }

                        _ => continue,
                    }

                    match type_conversion.arguments.first() {
                        Some(solidity::ast::Expression::Literal(solidity::ast::Literal {
                            value: Some(value),
                            kind: solidity::ast::LiteralKind::Number,
                            ..
                        })) if value == "0" => (),

                        _ => continue,
                    }

                    for (parameter_index, parameter) in
                        function_definition.parameters.parameters.iter().enumerate()
                    {
                        if parameter.id == parameter_identifier.referenced_declaration {
                            function_state.parameters_verified[parameter_index].2 = true;
                            break;
                        }
                    }
                }

                _ => {}
            }
        }

        Ok(())
    }
}
