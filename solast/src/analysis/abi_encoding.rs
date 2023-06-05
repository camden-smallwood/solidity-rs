use crate::report::Report;
use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct AbiEncodingVisitor {
    report: Rc<RefCell<Report>>,
    declaration_type_names: HashMap<NodeID, TypeName>
}

impl AbiEncodingVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self {
            report,
            declaration_type_names: HashMap::new(),
        }
    }

    fn add_report_entry(
        &mut self,
        source_unit_path: String,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        source_line: usize,
        expression: &dyn std::fmt::Display,
    ) {
        self.report.borrow_mut().add_entry(
            source_unit_path,
            Some(source_line),
            format!(
                "{} contains the potential for hash collisions: `{}`",
                contract_definition.definition_node_location(definition_node),
                expression,
            ),
        );
    }
}

impl AstVisitor for AbiEncodingVisitor {
    fn visit_variable_declaration<'a, 'b>(&mut self, context: &mut VariableDeclarationContext<'a, 'b>) -> std::io::Result<()> {
        //
        // Store the type of any variable declarations
        //

        if let Some(type_name) = context.variable_declaration.type_name.as_ref() {
            if self.declaration_type_names.contains_key(&context.variable_declaration.id) {
                return Ok(())
            }

            self.declaration_type_names.insert(context.variable_declaration.id, type_name.clone());
        }

        Ok(())
    }

    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> std::io::Result<()> {
        //
        // Only check for calls to abi.encodePacked(...)
        //

        if let Expression::MemberAccess(MemberAccess { expression, member_name, .. }) = context.function_call.expression.as_ref() {
            if let Expression::Identifier(Identifier { name, .. }) = expression.as_ref() {
                if name != "abi" || member_name != "encodePacked" {
                    return Ok(())
                }
            } else {
                return Ok(())
            }
        } else {
            return Ok(())
        }

        //
        // Only check if multiple arguments are supplied: abi.encodePacked(as, bs, ...)
        //

        if context.function_call.arguments.len() < 2 {
            return Ok(())
        }

        //
        // Determine if any parameters contain any variably-sized arrays
        //

        let mut any_arguments_variably_sized = false;

        for expression in context.function_call.arguments.iter() {
            if any_arguments_variably_sized {
                break;
            }

            for referenced_declaration in expression.referenced_declarations() {
                if let Some(TypeName::ArrayTypeName(ArrayTypeName { length: None, .. })) = self.declaration_type_names.get(&referenced_declaration) {
                    any_arguments_variably_sized = true;
                    break;
                }
            }
        }

        //
        // If so, print a message warning about potential hash collisions
        //

        if any_arguments_variably_sized {
            self.add_report_entry(
                context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
                context.contract_definition,
                context.definition_node,
                context.current_source_unit.source_line(context.function_call.src.as_str())?,
                context.function_call,
            );
        }

        Ok(())
    }
}
