use crate::report::Report;
use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::{cell::RefCell, collections::{HashMap, HashSet}, io, rc::Rc};

struct VariableInfo {
    assigned: bool,
    constant: bool,
}

struct ContractInfo {
    variable_info: HashMap<NodeID, VariableInfo>,
    variable_aliases: HashMap<NodeID, HashSet<NodeID>>,
}

pub struct StateVariableMutabilityVisitor {
    report: Rc<RefCell<Report>>,
    contract_info: HashMap<NodeID, ContractInfo>,
}

impl StateVariableMutabilityVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self {
            report,
            contract_info: HashMap::new(),
        }
    }
}

//
// TODO:
//   check for local variables which are bound to array state variable entries
//   if the local variable mutates state, don't suggest that the state variable should be immutable
//

impl AstVisitor for StateVariableMutabilityVisitor {
    fn visit_contract_definition<'a>(&mut self, context: &mut ContractDefinitionContext<'a>) -> io::Result<()> {
        self.contract_info.entry(context.contract_definition.id).or_insert_with(|| ContractInfo {
            variable_info: HashMap::new(),
            variable_aliases: HashMap::new(),
        });

        Ok(())
    }

    fn leave_contract_definition<'a>(&mut self, context: &mut ContractDefinitionContext<'a>) -> io::Result<()> {
        if let Some(contract_info) = self.contract_info.get(&context.contract_definition.id) {
            for (&id, variable_info) in contract_info.variable_info.iter() {
                if let Some(variable_declaration) = context.contract_definition.variable_declaration(id) {
                    if let Some(solidity::ast::Mutability::Constant | solidity::ast::Mutability::Immutable) = variable_declaration.mutability.as_ref() {
                        continue;
                    }

                    if variable_declaration.constant {
                        continue;
                    }

                    if let Some(TypeName::ElementaryTypeName(ElementaryTypeName {
                        name: type_name,
                        ..
                    })) = variable_declaration.type_name.as_ref() {
                        match type_name.as_str() {
                            "bytes" | "string" => continue,
                            _ => ()
                        }
                    }
                    
                    if !variable_info.assigned {
                        self.report.borrow_mut().add_entry(
                            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                            Some(context.current_source_unit.source_line(variable_declaration.src.as_str())?),
                            format!(
                                "The {} `{}.{}` {} state variable can be declared `{}`",
                                variable_declaration.visibility,
                                context.contract_definition.name,
                                variable_declaration.name,
                                variable_declaration.type_name.as_ref().unwrap(),
                                if variable_info.constant { "constant" } else { "immutable" }
                            ),
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn visit_variable_declaration<'a, 'b>(&mut self, context: &mut VariableDeclarationContext<'a, 'b>) -> io::Result<()> {
        let contract_definition = match context.contract_definition.as_ref() {
            Some(contract_definition) => contract_definition,
            None => return Ok(())
        };

        let contract_info = match self.contract_info.get_mut(&contract_definition.id) {
            Some(contract_info) => contract_info,
            None => return Ok(())
        };

        let definition_node = match context.definition_node.as_ref() {
            Some(definition_node) => definition_node,
            None => return Ok(())
        };

        match definition_node {
            ContractDefinitionNode::VariableDeclaration(_) => {
                contract_info.variable_info.entry(context.variable_declaration.id).or_insert_with(|| VariableInfo {
                    assigned: false,
                    constant: context.variable_declaration.value.is_some(),
                });

                contract_info.variable_aliases.entry(context.variable_declaration.id).or_insert_with(HashSet::new);
            }

            ContractDefinitionNode::FunctionDefinition(_) | ContractDefinitionNode::ModifierDefinition(_) => {
                if let StorageLocation::Storage = context.variable_declaration.storage_location {
                    if let Some(value) = context.variable_declaration.value.as_ref() {
                        for id in value.referenced_declarations() {
                            if let Some(variable_aliases) = contract_info.variable_aliases.get_mut(&id) {
                                if !variable_aliases.contains(&context.variable_declaration.id) {
                                    variable_aliases.insert(context.variable_declaration.id);
                                }
                            }
                        }
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn visit_assignment<'a, 'b>(&mut self, context: &mut AssignmentContext<'a, 'b>) -> io::Result<()> {
        if let ContractDefinitionNode::FunctionDefinition(FunctionDefinition {
            kind: FunctionKind::Constructor,
            ..
        }) = context.definition_node {
            return Ok(())
        }

        let contract_info = match self.contract_info.get_mut(&context.contract_definition.id) {
            Some(contract_info) => contract_info,
            None => return Ok(())
        };

        if let Expression::MemberAccess(member_access) = context.assignment.left_hand_side.as_ref() {
            let referenced_declarations = member_access.expression.referenced_declarations();
    
            if let Some(&referenced_declaration) = referenced_declarations.first() {
                if let Some((id, _)) = contract_info.variable_aliases.iter_mut().find(|(_, aliases)| aliases.contains(&referenced_declaration)) {
                    contract_info.variable_info.get_mut(id).unwrap().assigned = true;
                }
            }
        }

        let ids = context.contract_definition.get_assigned_state_variables(
            context.source_units,
            context.definition_node,
            context.assignment.left_hand_side.as_ref(),
        );

        for id in ids {
            if contract_info.variable_info.contains_key(&id) {
                contract_info.variable_info.get_mut(&id).unwrap().assigned = true;
            }
        }

        Ok(())
    }

    fn visit_unary_operation<'a, 'b>(&mut self, context: &mut UnaryOperationContext<'a, 'b>) -> io::Result<()> {
        if let ContractDefinitionNode::FunctionDefinition(FunctionDefinition {
            kind: FunctionKind::Constructor,
            ..
        }) = context.definition_node {
            return Ok(())
        }
        
        let ids = context.contract_definition.get_assigned_state_variables(
            context.source_units,
            context.definition_node,
            context.unary_operation.sub_expression.as_ref(),
        );

        for id in ids {
            let contract_info = match self.contract_info.get_mut(&context.contract_definition.id) {
                Some(contract_info) => contract_info,
                None => continue
            };

            if contract_info.variable_info.contains_key(&id) {
                contract_info.variable_info.get_mut(&id).unwrap().assigned = true;
            }
        }

        Ok(())
    }

    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> io::Result<()> {
        if let Expression::MemberAccess(member_access) = context.function_call.expression.as_ref() {
            if member_access.referenced_declaration.is_none() && (member_access.member_name == "push" || member_access.member_name == "pop") {
                if let ContractDefinitionNode::FunctionDefinition(FunctionDefinition {
                    kind: FunctionKind::Constructor,
                    ..
                }) = context.definition_node {
                    return Ok(())
                }
                
                let ids = context.contract_definition.get_assigned_state_variables(
                    context.source_units,
                    context.definition_node,
                    member_access.expression.as_ref(),
                );
                
                for id in ids {
                    let contract_info = match self.contract_info.get_mut(&context.contract_definition.id) {
                        Some(contract_info) => contract_info,
                        None => continue
                    };
        
                    if contract_info.variable_info.contains_key(&id) {
                        contract_info.variable_info.get_mut(&id).unwrap().assigned = true;
                    }
                }        
            }
        }
        
        Ok(())
    }
}
