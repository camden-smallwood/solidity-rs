use solidity::ast::*;
use std::io;
use yul::{InlineAssembly, YulAssignment, YulBlock, YulCase, YulExpression, YulExpressionStatement, YulFunctionCall, YulIdentifier, YulLiteral, YulStatement, YulSwitch, YulVariableDeclaration};

#[allow(unused_variables)]
pub trait AstVisitor {
    fn visit_source_unit(&mut self, source_unit: &SourceUnit) -> io::Result<()> {
        Ok(())
    }

    fn leave_source_unit(&mut self, source_unit: &SourceUnit) -> io::Result<()> {
        Ok(())
    }

    fn visit_pragma_directive(
        &mut self,
        source_unit: &SourceUnit,
        pragma_directive: &PragmaDirective,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_import_directive(
        &mut self,
        source_unit: &SourceUnit,
        import_directive: &ImportDirective,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_struct_definition(
        &mut self,
        source_unit: &SourceUnit,
        struct_definition: &StructDefinition,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_enum_definition(
        &mut self,
        source_unit: &SourceUnit,
        enum_definition: &EnumDefinition,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_contract_definition(
        &mut self,
        source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
    ) -> io::Result<()> {
        Ok(())
    }

    fn leave_contract_definition(
        &mut self,
        source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_using_for_directive(
        &mut self,
        source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        using_for_directive: &UsingForDirective,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_variable_declaration<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        variable_declaration: &'a VariableDeclaration,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_event_definition(
        &mut self,
        source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        event_definition: &EventDefinition,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_error_definition(
        &mut self,
        source_unit: &SourceUnit,
        error_definition: &ErrorDefinition,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_modifier_definition(
        &mut self,
        source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        modifier_definition: &ModifierDefinition,
    ) -> io::Result<()> {
        Ok(())
    }

    fn leave_modifier_definition(
        &mut self,
        source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        modifier_definition: &ModifierDefinition,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_function_definition(
        &mut self,
        source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        function_definition: &FunctionDefinition,
    ) -> io::Result<()> {
        Ok(())
    }

    fn leave_function_definition(
        &mut self,
        source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        function_definition: &FunctionDefinition,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_block<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        block: &'a Block,
    ) -> io::Result<()> {
        Ok(())
    }

    fn leave_block<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        block: &'a Block,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_variable_declaration_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        variable_declaration_statement: &'a VariableDeclarationStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_if_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        if_statement: &'a IfStatement
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_for_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        for_statement: &'a ForStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn leave_for_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        for_statement: &'a ForStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_while_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        while_statement: &'a WhileStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn leave_while_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        while_statement: &'a WhileStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_emit_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        emit_statement: &'a EmitStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_try_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        try_statement: &'a TryStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_revert_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        revert_statement: &'a RevertStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_block_or_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        block_or_statement: &'a BlockOrStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_return<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        return_statement: &'a Return,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_expression<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        expression: &'a Expression
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_literal(
        &mut self,
        source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        blocks: &mut Vec<&Block>,
        statement: Option<&Statement>,
        literal: &Literal,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_identifier(
        &mut self,
        source_unit: &SourceUnit,
        contract_definition: &ContractDefinition,
        definition_node: &ContractDefinitionNode,
        blocks: &mut Vec<&Block>,
        statement: Option<&Statement>,
        identifier: &Identifier,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_unary_operation<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        unary_operation: &'a UnaryOperation
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_binary_operation<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        binary_operation: &'a BinaryOperation
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_conditional<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        conditional: &'a Conditional,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_assignment<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        assignment: &'a Assignment,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_function_call<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        function_call: &'a FunctionCall,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_function_call_options<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        function_call_options: &'a FunctionCallOptions,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_index_access<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        index_access: &'a IndexAccess,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_index_range_access<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        index_range_access: &'a IndexRangeAccess,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_member_access<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        member_access: &'a MemberAccess,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_elementary_type_name_expression<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        elementary_type_name_expression: &'a ElementaryTypeNameExpression,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_tuple_expression<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        tuple_expression: &'a TupleExpression,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_new_expression<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: Option<&'a Statement>,
        new_expression: &'a NewExpression,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_unhandled_statement(
        &mut self,
        source_unit: &SourceUnit,
        node_type: &NodeType,
        src: &Option<String>,
        id: &Option<NodeID>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_unhandled_expression(
        &mut self,
        source_unit: &SourceUnit,
        node_type: &NodeType,
        src: &Option<String>,
        id: &Option<NodeID>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_inline_assembly<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_block<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_block: &'a YulBlock,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: &'a YulStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_switch<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: &'a YulStatement,
        yul_switch: &'a YulSwitch,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_case<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: &'a YulStatement,
        yul_switch: &'a YulSwitch,
        yul_case: &'a YulCase,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_assignment<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: &'a YulStatement,
        yul_assignment: &'a YulAssignment,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_variable_declaration<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: &'a YulStatement,
        yul_variable_declaration: &'a YulVariableDeclaration,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_expression_statement<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: &'a YulStatement,
        yul_expression_statement: &'a YulExpressionStatement,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_expression<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: Option<&'a YulStatement>,
        yul_expression: &'a YulExpression,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_literal<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: Option<&'a YulStatement>,
        yul_expression: &'a YulExpression,
        yul_literal: &'a YulLiteral,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_identifier<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: Option<&'a YulStatement>,
        yul_expression: Option<&'a YulExpression>,
        yul_identifier: &'a YulIdentifier,
    ) -> io::Result<()> {
        Ok(())
    }

    fn visit_yul_function_call<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: Option<&'a YulStatement>,
        yul_expression: &'a YulExpression,
        yul_function_call: &'a YulFunctionCall,
    ) -> io::Result<()> {
        Ok(())
    }
}
