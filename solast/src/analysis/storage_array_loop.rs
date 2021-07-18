use super::{AstVisitor, ForStatementContext, FunctionDefinitionContext, VariableDeclarationContext};
use solidity::ast::*;
use std::{
    collections::{HashMap, HashSet},
    io,
};

struct FunctionInfo {
    loops_over_storage_array: bool,
}

pub struct StorageArrayLoopVisitor {
    storage_arrays: HashSet<NodeID>,
    functions: HashMap<NodeID, FunctionInfo>,
}

impl Default for StorageArrayLoopVisitor {
    fn default() -> Self {
        Self {
            storage_arrays: HashSet::new(),
            functions: HashMap::new(),
        }
    }
}

impl StorageArrayLoopVisitor {
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

impl AstVisitor for StorageArrayLoopVisitor {
    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if !self.functions.contains_key(&context.function_definition.id) {
            self.functions.insert(
                context.function_definition.id,
                FunctionInfo {
                    loops_over_storage_array: false,
                },
            );
        }

        for variable_declaration in context.function_definition.parameters.parameters.iter() {
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

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        if let Some(function_info) = self.functions.get(&context.function_definition.id) {
            if function_info.loops_over_storage_array {
                println!(
                    "\tL{}: {} {} {} performs a loop over a storage array, querying the length over each iteration",

                    context.current_source_unit.source_line(context.function_definition.src.as_str()).unwrap(),

                    format!("{:?}", context.function_definition.visibility),

                    if context.function_definition.name.is_empty() {
                        format!("{}", context.contract_definition.name)
                    } else {
                        format!("{}.{}", context.contract_definition.name, context.function_definition.name)
                    },

                    context.function_definition.kind
                );
            }
        }

        Ok(())
    }

    fn visit_variable_declaration<'a, 'b>(&mut self, context: &mut VariableDeclarationContext<'a, 'b>) -> io::Result<()> {
        let storage_location = match &context.variable_declaration.storage_location {
            solidity::ast::StorageLocation::Default if context.variable_declaration.state_variable => solidity::ast::StorageLocation::Storage,
            storage_location => storage_location.clone(),
        };

        if let solidity::ast::StorageLocation::Storage = storage_location {
            if let Some(solidity::ast::TypeName::ArrayTypeName(_)) = context.variable_declaration.type_name {
                if !self.storage_arrays.contains(&context.variable_declaration.id) {
                    self.storage_arrays.insert(context.variable_declaration.id);
                }
            }
        }

        Ok(())
    }

    fn visit_for_statement<'a, 'b>(&mut self, context: &mut ForStatementContext<'a, 'b>) -> io::Result<()> {
        let definition_id = match context.definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        if let Some(expression) = context.for_statement.condition.as_ref() {
            if self.expression_contains_storage_array_length(expression) {
                self.functions
                    .get_mut(&definition_id)
                    .unwrap()
                    .loops_over_storage_array = true;
            }
        }

        Ok(())
    }

    fn visit_while_statement<'a, 'b>(&mut self, context: &mut super::WhileStatementContext<'a, 'b>) -> io::Result<()> {
        let definition_id = match context.definition_node {
            solidity::ast::ContractDefinitionNode::FunctionDefinition(definition) => definition.id,
            solidity::ast::ContractDefinitionNode::ModifierDefinition(definition) => definition.id,
            _ => return Ok(())
        };

        if self.expression_contains_storage_array_length(&context.while_statement.condition) {
            self.functions
                .get_mut(&definition_id)
                .unwrap()
                .loops_over_storage_array = true;
        }

        Ok(())
    }
}
