use super::AstVisitor;
use std::io;

pub struct ComparisonUtilizationVisitor;

impl AstVisitor for ComparisonUtilizationVisitor {
    fn visit_if_statement<'a>(
        &mut self,
        _source_unit: &'a solidity::ast::SourceUnit,
        _contract_definition: &'a solidity::ast::ContractDefinition,
        _definition_node: &'a solidity::ast::ContractDefinitionNode,
        _blocks: &mut Vec<&'a solidity::ast::Block>,
        _if_statement: &'a solidity::ast::IfStatement,
    ) -> io::Result<()> {
        //
        // TODO:
        //
        // Scenario:
        //   if (x > 0) {
        //     doSomething(y);
        //   } else {
        //     doSomething(z);
        //   }
        //
        // Description:
        //   Verify `x` is utilized within the `true` or `false` blocks.
        //   Unless `y` or `z` is bound to `x`, then `x` goes unutilized, which can be unintentional.
        //

        Ok(())
    }
}
