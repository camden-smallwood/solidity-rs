use super::AstVisitor;
use std::io;

use solidity::ast::{
    Block, ContractDefinition, ContractDefinitionNode, ElementaryTypeNameExpression, Expression,
    FunctionCall, FunctionCallKind, SourceUnit, Statement, TypeName,
};

pub struct UncheckedCastingVisitor;

impl AstVisitor for UncheckedCastingVisitor {
    fn visit_function_call<'a>(
        &mut self,
        _source_unit: &'a SourceUnit,
        _contract_definition: &'a ContractDefinition,
        _definition_node: &'a ContractDefinitionNode,
        _blocks: &mut Vec<&'a Block>,
        _statement: Option<&'a Statement>,
        function_call: &'a FunctionCall,
    ) -> io::Result<()> {
        if function_call.kind != FunctionCallKind::TypeConversion {
            return Ok(())
        }

        let type_name = match function_call.expression.as_ref() {
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_name: TypeName::ElementaryTypeName(type_name),
                ..
            }) => type_name,

            _ => return Ok(())
        };

        if type_name.name.starts_with("int") || type_name.name.starts_with("uint") {
            //
            // TODO: if the argument is a non-literal, verify its expression was
            //       checked for validity via require, if/else or a conditional
            //
        }

        Ok(())
    }
}
