use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::{collections::{HashMap, HashSet}, io};

struct FunctionInfo {
    assigned_return_variables: HashSet<NodeID>,
}

#[derive(Default)]
pub struct MissingReturnVisitor {
    function_info: HashMap<NodeID, FunctionInfo>,
}

impl MissingReturnVisitor {
    fn print_message(
        &mut self,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize
    ) {
        println!(
            "\t{} is missing an explicit return statement",
            contract_definition.definition_node_location(source_line, definition_node),
        );
    }
}

impl AstVisitor for MissingReturnVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if context.function_definition.return_parameters.parameters.is_empty() {
            return Ok(());
        }

        if context.function_definition.body.is_none() {
            return Ok(());
        }

        self.function_info.entry(context.function_definition.id).or_insert_with(|| FunctionInfo {
            assigned_return_variables: HashSet::new(),
        });

        Ok(())
    }

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        let function_info = match self.function_info.get(&context.function_definition.id) {
            Some(function_info) => function_info,
            None => return Ok(())
        };

        if BlockOrStatement::Block(Box::new(context.function_definition.body.as_ref().unwrap().clone())).contains_returns() {
            return Ok(());
        }

        let mut assigned = vec![];

        for variable_declaration in context.function_definition.return_parameters.parameters.iter() {
            assigned.push(function_info.assigned_return_variables.contains(&variable_declaration.id));
        }

        if assigned.iter().all(|assigned| !assigned) {
            self.print_message(
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.function_definition.src.as_str())?
            );
        }

        Ok(())
    }

    fn visit_assignment<'a, 'b>(&mut self, context: &mut AssignmentContext<'a, 'b>) -> io::Result<()> {
        let function_definition = match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(())
        };

        let function_info = match self.function_info.get_mut(&function_definition.id) {
            Some(function_info) => function_info,
            _ => return Ok(())
        };

        for id in function_definition.get_assigned_return_variables(context.assignment.left_hand_side.as_ref()) {
            if !function_info.assigned_return_variables.contains(&id) {
                function_info.assigned_return_variables.insert(id);
            }
        }

        Ok(())
    }

    fn visit_yul_assignment<'a, 'b, 'c>(&mut self, context: &mut YulAssignmentContext<'a, 'b, 'c>) -> io::Result<()> {
        let function_definition = match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(function_definition) => function_definition,
            _ => return Ok(())
        };

        let function_info = match self.function_info.get_mut(&function_definition.id) {
            Some(function_info) => function_info,
            _ => return Ok(())
        };

        for yul_identifier in context.yul_assignment.variable_names.iter() {
            if let Some(variable_declaration) = function_definition.return_parameters.parameters.iter().find(|p| p.name == yul_identifier.name) {
                if !function_info.assigned_return_variables.contains(&variable_declaration.id) {
                    function_info.assigned_return_variables.insert(variable_declaration.id);
                }
            }
        }
        
        Ok(())
    }
}
