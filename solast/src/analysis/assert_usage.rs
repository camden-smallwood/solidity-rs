use crate::report::Report;
use eth_lang_utils::ast::*;
use solidity::ast::*;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

pub struct AssertUsageVisitor {
    report: Rc<RefCell<Report>>,
    reported_definitions: HashSet<NodeID>,
}

impl AssertUsageVisitor {
    pub fn new(report: Rc<RefCell<Report>>) -> Self {
        Self {
            report,
            reported_definitions: HashSet::new(),
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
                "{} contains assert usage: `{}`",
                contract_definition.definition_node_location(definition_node),
                expression,
            ),
        );
    }
}

impl AstVisitor for AssertUsageVisitor {
    fn visit_function_call<'a, 'b>(&mut self, context: &mut FunctionCallContext<'a, 'b>) -> std::io::Result<()> {
        //
        // Get the identifier associated with the function or modifier containing the function call
        //

        let definition_id = match context.definition_node {
            ContractDefinitionNode::FunctionDefinition(FunctionDefinition { id, .. }) |
            ContractDefinitionNode::ModifierDefinition(ModifierDefinition { id, .. }) => *id,

            _ => return Ok(())
        };

        //
        // Check if the expression of function call is the "assert" identifier
        //

        let is_assert = match context.function_call.expression.as_ref() {
            Expression::Identifier(Identifier { name, .. }) => name == "assert",
            _ => false
        };

        if !is_assert {
            return Ok(())
        }

        //
        // Don't display multiple messages for the same function or modifier
        //

        if self.reported_definitions.contains(&definition_id) {
            return Ok(())
        }

        self.reported_definitions.insert(definition_id);

        //
        // Print a message about the assert usage
        //

        self.add_report_entry(
            context.current_source_unit.absolute_path.clone().unwrap_or_else(String::new),
            context.contract_definition,
            context.definition_node,
            context.current_source_unit.source_line(context.function_call.src.as_str())?,
            context.function_call,
        );
        
        Ok(())
    }
}
