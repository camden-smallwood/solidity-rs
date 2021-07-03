use solidity::ast::*;
use std::{collections::HashSet, io};
use yul::{
    InlineAssembly, YulAssignment, YulBlock, YulCase, YulExpression, YulExpressionStatement,
    YulFunctionCall, YulIdentifier, YulIf, YulLiteral, YulStatement, YulSwitch,
    YulVariableDeclaration,
};

pub struct SourceUnitContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
}

impl<'a> SourceUnitContext<'a> {
    pub fn create_pragma_directive_context(&self, pragma_directive: &'a PragmaDirective) -> PragmaDirectiveContext<'a> {
        PragmaDirectiveContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            pragma_directive
        }
    }

    pub fn create_import_directive_context(&self, import_directive: &'a ImportDirective) -> ImportDirectiveContext<'a> {
        ImportDirectiveContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            import_directive
        }
    }

    pub fn create_contract_definition_context(&self, contract_definition: &'a ContractDefinition) -> ContractDefinitionContext<'a> {
        ContractDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition
        }
    }

    pub fn create_struct_definition_context(&self, struct_definition: &'a StructDefinition) -> StructDefinitionContext<'a> {
        StructDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: None,
            struct_definition
        }
    }

    pub fn create_enum_definition_context(&self, enum_definition: &'a EnumDefinition) -> EnumDefinitionContext<'a> {
        EnumDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: None,
            enum_definition
        }
    }
}

pub struct PragmaDirectiveContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub pragma_directive: &'a PragmaDirective,
}

pub struct ImportDirectiveContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub import_directive: &'a ImportDirective,
}

pub struct ContractDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
}

impl<'a> ContractDefinitionContext<'a> {
    pub fn create_using_for_directive_context(&self, using_for_directive: &'a UsingForDirective) -> UsingForDirectiveContext<'a> {
        UsingForDirectiveContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: self.contract_definition,
            using_for_directive
        }
    }

    pub fn create_struct_definition_context(&self, struct_definition: &'a StructDefinition) -> StructDefinitionContext<'a> {
        StructDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: Some(self.contract_definition),
            struct_definition
        }
    }

    pub fn create_enum_definition_context(&self, enum_definition: &'a EnumDefinition) -> EnumDefinitionContext<'a> {
        EnumDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: Some(self.contract_definition),
            enum_definition
        }
    }
}

pub struct StructDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: Option<&'a ContractDefinition>,
    pub struct_definition: &'a StructDefinition,
}

pub struct EnumDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: Option<&'a ContractDefinition>,
    pub enum_definition: &'a EnumDefinition,
}

pub struct UsingForDirectiveContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub using_for_directive: &'a UsingForDirective,
}

pub struct VariableDeclarationContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub variable_declaration: &'a VariableDeclaration,
}

pub struct EventDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub event_definition: &'a EventDefinition,
}

pub struct ErrorDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub error_definition: &'a ErrorDefinition,
}

pub struct ModifierDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub modifier_definition: &'a ModifierDefinition,
}

pub struct FunctionDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub function_definition: &'a FunctionDefinition,
}

pub struct BlockContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub block: &'a Block,
}

pub struct StatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: &'a Statement,
}

pub struct VariableDeclarationStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub variable_declaration_statement: &'a VariableDeclarationStatement,
}

pub struct IfStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub if_statement: &'a IfStatement,
}

pub struct ForStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub for_statement: &'a ForStatement,
}

pub struct WhileStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub while_statement: &'a WhileStatement,
}

pub struct EmitStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub emit_statement: &'a EmitStatement,
}

pub struct TryStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub try_statement: &'a TryStatement,
}

pub struct RevertStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub revert_statement: &'a RevertStatement,
}

pub struct BlockOrStatementContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub block_or_statement: &'a BlockOrStatement,
}

pub struct ReturnContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub return_statement: &'a Return,
}

pub struct ExpressionContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub expression: &'a Expression,
}

pub struct LiteralContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub literal: &'a Literal,
}

pub struct IdentifierContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub identifier: &'a Identifier,
}

pub struct UnaryOperationContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub unary_operation: &'a UnaryOperation,
}

pub struct BinaryOperationContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub binary_operation: &'a BinaryOperation,
}

pub struct ConditionalContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub conditional: &'a Conditional,
}

pub struct AssignmentContext<'a, 'b> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub blocks: &'b mut Vec<&'a Block>,
    pub statement: Option<&'a Statement>,
    pub assignment: &'a Assignment,
}

#[allow(unused_variables)]
pub trait AstVisitor {
    fn visit_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_pragma_directive<'a>(&mut self, context: &mut PragmaDirectiveContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_pragma_directive<'a>(&mut self, context: &mut PragmaDirectiveContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_import_directive<'a>(&mut self, context: &mut ImportDirectiveContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_import_directive<'a>(&mut self, context: &mut ImportDirectiveContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_struct_definition<'a>(&mut self, context: &mut StructDefinitionContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_struct_definition<'a>(&mut self, context: &mut StructDefinitionContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_enum_definition<'a>(&mut self, context: &mut EnumDefinitionContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_enum_definition<'a>(&mut self, context: &mut EnumDefinitionContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_contract_definition<'a>(&mut self, context: &mut ContractDefinitionContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_contract_definition<'a>(&mut self, context: &mut ContractDefinitionContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_using_for_directive<'a>(&mut self, context: &mut UsingForDirectiveContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_using_for_directive<'a>(&mut self, context: &mut UsingForDirectiveContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_variable_declaration<'a, 'b>(&mut self, context: &mut VariableDeclarationContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_variable_declaration<'a, 'b>(&mut self, context: &mut VariableDeclarationContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_event_definition<'a>(&mut self, context: &mut EventDefinitionContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_event_definition<'a>(&mut self, context: &mut EventDefinitionContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_error_definition<'a>(&mut self, context: &mut ErrorDefinitionContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_error_definition<'a>(&mut self, context: &mut ErrorDefinitionContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_modifier_definition<'a>(&mut self, context: &mut ModifierDefinitionContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_modifier_definition<'a>(&mut self, context: &mut ModifierDefinitionContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> { Ok(()) }
    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> { Ok(()) }

    fn visit_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_variable_declaration_statement<'a, 'b>(&mut self, context: &mut VariableDeclarationStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_variable_declaration_statement<'a, 'b>(&mut self, context: &mut VariableDeclarationStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_if_statement<'a, 'b>(&mut self, context: &mut IfStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_if_statement<'a, 'b>(&mut self, context: &mut IfStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_for_statement<'a, 'b>(&mut self, context: &mut ForStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_for_statement<'a, 'b>(&mut self, context: &mut ForStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_while_statement<'a, 'b>(&mut self, context: &mut WhileStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_while_statement<'a, 'b>(&mut self, context: &mut WhileStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_emit_statement<'a, 'b>(&mut self, context: &mut EmitStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_emit_statement<'a, 'b>(&mut self, context: &mut EmitStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_try_statement<'a, 'b>(&mut self, context: &mut TryStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_try_statement<'a, 'b>(&mut self, context: &mut TryStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_revert_statement<'a, 'b>(&mut self, context: &mut RevertStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_revert_statement<'a, 'b>(&mut self, context: &mut RevertStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_block_or_statement<'a, 'b>(&mut self, context: &mut BlockOrStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_block_or_statement<'a, 'b>(&mut self, context: &mut BlockOrStatementContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_return<'a, 'b>(&mut self, context: &mut ReturnContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_return<'a, 'b>(&mut self, context: &mut ReturnContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_expression<'a, 'b>(&mut self, context: &mut ExpressionContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_expression<'a, 'b>(&mut self, context: &mut ExpressionContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_identifier<'a, 'b>(&mut self, context: &mut IdentifierContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_identifier<'a, 'b>(&mut self, context: &mut IdentifierContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_unary_operation<'a, 'b>(&mut self, context: &mut UnaryOperationContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_unary_operation<'a, 'b>(&mut self, context: &mut UnaryOperationContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_binary_operation<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_binary_operation<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_conditional<'a, 'b>(&mut self, context: &mut ConditionalContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_conditional<'a, 'b>(&mut self, context: &mut ConditionalContext<'a, 'b>) -> io::Result<()> { Ok(()) }

    fn visit_assignment<'a, 'b>(&mut self, context: &mut AssignmentContext<'a, 'b>) -> io::Result<()> { Ok(()) }
    fn leave_assignment<'a, 'b>(&mut self, context: &mut AssignmentContext<'a, 'b>) -> io::Result<()> { Ok(()) }

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

    fn visit_yul_if<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: &'a YulStatement,
        yul_if: &'a YulIf,
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

struct AstVisitorData<'a> {
    pub analyzed_paths: HashSet<String>,
    pub visitors: Vec<Box<dyn AstVisitor + 'a>>,
}

impl AstVisitor for AstVisitorData<'_> {
    fn visit_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_source_unit(context)?;
        }

        for node in context.current_source_unit.nodes.iter() {
            match node {
                SourceUnitNode::PragmaDirective(pragma_directive) => {
                    let mut context = context.create_pragma_directive_context(pragma_directive);
                    self.visit_pragma_directive(&mut context)?;
                    self.leave_pragma_directive(&mut context)?;
                }

                SourceUnitNode::ImportDirective(import_directive) => {
                    let mut context = context.create_import_directive_context(import_directive);
                    self.visit_import_directive(&mut context)?;
                    self.leave_import_directive(&mut context)?;
                }

                SourceUnitNode::ContractDefinition(contract_definition) => {
                    let mut context = context.create_contract_definition_context(contract_definition);
                    self.visit_contract_definition(&mut context)?;
                    self.leave_contract_definition(&mut context)?;
                }

                SourceUnitNode::StructDefinition(struct_definition) => {
                    let mut context = context.create_struct_definition_context(struct_definition);
                    self.visit_struct_definition(&mut context)?;
                    self.leave_struct_definition(&mut context)?;
                }

                SourceUnitNode::EnumDefinition(enum_definition) => {
                    let mut context = context.create_enum_definition_context(enum_definition);
                    self.visit_enum_definition(&mut context)?;
                    self.leave_enum_definition(&mut context)?;
                }
            }
        }

        Ok(())
    }

    fn leave_source_unit<'a>(&mut self, context: &mut SourceUnitContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_source_unit(context)?;
        }

        Ok(())
    }

    fn visit_pragma_directive<'a>(&mut self, context: &mut PragmaDirectiveContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_pragma_directive(context)?;
        }

        Ok(())
    }

    fn leave_pragma_directive<'a>(&mut self, context: &mut PragmaDirectiveContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_pragma_directive(context)?;
        }

        Ok(())
    }

    fn visit_import_directive<'a>(&mut self, context: &mut ImportDirectiveContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_import_directive(context)?;
        }

        Ok(())
    }

    fn leave_import_directive<'a>(&mut self, context: &mut ImportDirectiveContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_import_directive(context)?;
        }

        Ok(())
    }

    fn visit_struct_definition<'a>(&mut self, context: &mut StructDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_struct_definition(context)?;
        }

        Ok(())
    }

    fn leave_struct_definition<'a>(&mut self, context: &mut StructDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_struct_definition(context)?;
        }

        Ok(())
    }

    fn visit_enum_definition<'a>(&mut self, context: &mut EnumDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_enum_definition(context)?;
        }

        Ok(())
    }

    fn leave_enum_definition<'a>(&mut self, context: &mut EnumDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_enum_definition(context)?;
        }

        Ok(())
    }

    fn visit_contract_definition<'a>(&mut self, context: &mut ContractDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_contract_definition(context)?;
        }

        for definition_node in context.contract_definition.nodes.iter() {
            match definition_node {
                ContractDefinitionNode::UsingForDirective(using_for_directive) => {
                    let mut context = UsingForDirectiveContext {
                        source_units: context.source_units,
                        current_source_unit: context.current_source_unit,
                        contract_definition: context.contract_definition,
                        using_for_directive
                    };

                    self.visit_using_for_directive(&mut context)?;
                    self.leave_using_for_directive(&mut context)?;
                }

                ContractDefinitionNode::StructDefinition(struct_definition) => {
                    let mut context = StructDefinitionContext {
                        source_units: context.source_units,
                        current_source_unit: context.current_source_unit,
                        contract_definition: Some(context.contract_definition),
                        struct_definition
                    };

                    self.visit_struct_definition(&mut context)?;
                    self.leave_struct_definition(&mut context)?;
                }

                ContractDefinitionNode::EnumDefinition(enum_definition) => {
                    let mut context = EnumDefinitionContext {
                        source_units: context.source_units,
                        current_source_unit: context.current_source_unit,
                        contract_definition: Some(context.contract_definition),
                        enum_definition
                    };

                    self.visit_enum_definition(&mut context)?;
                    self.leave_enum_definition(&mut context)?;
                }

                ContractDefinitionNode::VariableDeclaration(variable_declaration) => {
                    let mut context = VariableDeclarationContext {
                        source_units: context.source_units,
                        current_source_unit: context.current_source_unit,
                        contract_definition: context.contract_definition,
                        definition_node,
                        blocks: &mut vec![],
                        variable_declaration
                    };
                    
                    self.visit_variable_declaration(&mut context)?;
                    self.leave_variable_declaration(&mut context)?;
                }

                ContractDefinitionNode::EventDefinition(event_definition) => {
                    let mut context = EventDefinitionContext {
                        source_units: context.source_units,
                        current_source_unit: context.current_source_unit,
                        contract_definition: context.contract_definition,
                        event_definition
                    };

                    self.visit_event_definition(&mut context)?;
                    self.leave_event_definition(&mut context)?;
                }

                ContractDefinitionNode::ErrorDefinition(error_definition) => {
                    let mut context = ErrorDefinitionContext {
                        source_units: context.source_units,
                        current_source_unit: context.current_source_unit,
                        contract_definition: context.contract_definition,
                        error_definition
                    };

                    self.visit_error_definition(&mut context)?;
                    self.leave_error_definition(&mut context)?;
                }

                ContractDefinitionNode::FunctionDefinition(function_definition) => {
                    let mut context = FunctionDefinitionContext {
                        source_units: context.source_units,
                        current_source_unit: context.current_source_unit,
                        contract_definition: context.contract_definition,
                        definition_node: definition_node,
                        function_definition
                    };

                    self.visit_function_definition(&mut context)?;
                    self.leave_function_definition(&mut context)?;
                }

                ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                    let mut context = ModifierDefinitionContext {
                        source_units: context.source_units,
                        current_source_unit: context.current_source_unit,
                        contract_definition: context.contract_definition,
                        definition_node: definition_node,
                        modifier_definition
                    };

                    self.visit_modifier_definition(&mut context)?;
                    self.leave_modifier_definition(&mut context)?;
                }
            }
        }

        Ok(())
    }

    fn leave_contract_definition<'a>(&mut self, context: &mut ContractDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_contract_definition(context)?;
        }

        Ok(())
    }

    fn visit_using_for_directive<'a>(&mut self, context: &mut UsingForDirectiveContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_using_for_directive(context)?;
        }

        Ok(())
    }

    fn leave_using_for_directive<'a>(&mut self, context: &mut UsingForDirectiveContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_using_for_directive(context)?;
        }

        Ok(())
    }

    fn visit_variable_declaration<'a, 'b>(&mut self, context: &mut VariableDeclarationContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_variable_declaration(context)?;
        }

        Ok(())
    }

    fn leave_variable_declaration<'a, 'b>(&mut self, context: &mut VariableDeclarationContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_variable_declaration(context)?;
        }

        Ok(())
    }

    fn visit_event_definition<'a>(&mut self, context: &mut EventDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_event_definition(context)?;
        }

        Ok(())
    }

    fn leave_event_definition<'a>(&mut self, context: &mut EventDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_event_definition(context)?;
        }

        Ok(())
    }

    fn visit_error_definition<'a>(&mut self, context: &mut ErrorDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_error_definition(context)?;
        }

        Ok(())
    }

    fn leave_error_definition<'a>(&mut self, context: &mut ErrorDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_error_definition(context)?;
        }

        Ok(())
    }

    fn visit_modifier_definition<'a>(&mut self, context: &mut ModifierDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_modifier_definition(context)?;
        }

        let mut blocks = vec![];

        let mut context = BlockContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: &mut blocks,
            block: &context.modifier_definition.body,
        };

        self.visit_block(&mut context)?;
        self.leave_block(&mut context)?;

        Ok(())
    }

    fn leave_modifier_definition<'a>(&mut self, context: &mut ModifierDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_modifier_definition(context)?;
        }

        Ok(())
    }

    fn visit_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_function_definition(context)?;
        }

        if let Some(block) = context.function_definition.body.as_ref() {
            let mut blocks = vec![];

            let mut context = BlockContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: &mut blocks,
                block
            };
    
            self.visit_block(&mut context)?;
            self.leave_block(&mut context)?;
        }

        Ok(())
    }

    fn leave_function_definition<'a>(&mut self, context: &mut FunctionDefinitionContext<'a>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_function_definition(context)?;
        }

        Ok(())
    }

    fn visit_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_block(context)?;
        }

        context.blocks.push(context.block);

        for statement in context.block.statements.iter() {
            let mut context = StatementContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement
            };

            self.visit_statement(&mut context)?;
            self.leave_statement(&mut context)?;
        }

        Ok(())
    }

    fn leave_block<'a, 'b>(&mut self, context: &mut BlockContext<'a, 'b>) -> io::Result<()> {
        context.blocks.pop();

        for visitor in self.visitors.iter_mut() {
            visitor.leave_block(context)?;
        }

        Ok(())
    }

    fn visit_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_statement(context)?;
        }

        match context.statement {
            Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                let mut context = VariableDeclarationStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    variable_declaration_statement
                };

                self.visit_variable_declaration_statement(&mut context)?;
                self.leave_variable_declaration_statement(&mut context)?;
            }

            Statement::IfStatement(if_statement) => {
                let mut context = IfStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    if_statement
                };

                self.visit_if_statement(&mut context)?;
                self.leave_if_statement(&mut context)?;
            }

            Statement::ForStatement(for_statement) => {
                let mut context = ForStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    for_statement
                };

                self.visit_for_statement(&mut context)?;
                self.leave_for_statement(&mut context)?;
            }

            Statement::WhileStatement(while_statement) => {
                let mut context = WhileStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    while_statement
                };

                self.visit_while_statement(&mut context)?;
                self.leave_while_statement(&mut context)?;
            }

            Statement::EmitStatement(emit_statement) => {
                let mut context = EmitStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    emit_statement
                };
                
                self.visit_emit_statement(&mut context)?;
                self.leave_emit_statement(&mut context)?;
            }

            Statement::TryStatement(try_statement) => {
                let mut context = TryStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    try_statement
                };

                self.visit_try_statement(&mut context)?;
                self.leave_try_statement(&mut context)?;
            }

            Statement::RevertStatement(revert_statement) => {
                let mut context = RevertStatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    revert_statement
                };

                self.visit_revert_statement(&mut context)?;
                self.leave_revert_statement(&mut context)?;
            }

            Statement::UncheckedBlock(block) => {
                let mut context = BlockContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    block,
                };
                
                self.visit_block(&mut context)?;
                self.leave_block(&mut context)?;
            }

            Statement::Return(return_statement) => {
                let mut context = ReturnContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: Some(context.statement),
                    return_statement,
                };

                self.visit_return(&mut context)?;
                self.leave_return(&mut context)?;
            }

            Statement::ExpressionStatement(expression_statement) => {
                let mut context = ExpressionContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: Some(context.statement),
                    expression: &expression_statement.expression,
                };

                self.visit_expression(&mut context)?;
                self.leave_expression(&mut context)?;
            }

            Statement::InlineAssembly(inline_assembly) => {
                self.visit_inline_assembly(
                    context.current_source_unit,
                    context.contract_definition,
                    context.definition_node,
                    context.blocks,
                    context.statement,
                    inline_assembly,
                )?;
            }

            Statement::UnhandledStatement { node_type, src, id } => {
                self.visit_unhandled_statement(context.current_source_unit, node_type, src, id)?;
            }
        }

        Ok(())
    }

    fn leave_statement<'a, 'b>(&mut self, context: &mut StatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_statement(context)?;
        }

        Ok(())
    }

    fn visit_variable_declaration_statement<'a, 'b>(&mut self, context: &mut VariableDeclarationStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_variable_declaration_statement(context)?;
        }

        if let Some(initial_value) = context.variable_declaration_statement.initial_value.as_ref() {
            let mut context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: None,
                expression: initial_value,
            };

            self.visit_expression(&mut context)?;
            self.leave_expression(&mut context)?;
        }

        Ok(())
    }

    fn leave_variable_declaration_statement<'a, 'b>(&mut self, context: &mut VariableDeclarationStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_variable_declaration_statement(context)?;
        }

        Ok(())
    }

    fn visit_if_statement<'a, 'b>(&mut self, context: &mut IfStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_if_statement(context)?;
        }

        let mut true_body_context = BlockOrStatementContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            block_or_statement: &context.if_statement.true_body,
        };

        self.visit_block_or_statement(&mut true_body_context)?;
        self.leave_block_or_statement(&mut true_body_context)?;

        if let Some(false_body) = context.if_statement.false_body.as_ref() {
            let mut false_body_context = BlockOrStatementContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                block_or_statement: false_body,
            };
    
            self.visit_block_or_statement(&mut false_body_context)?;
            self.leave_block_or_statement(&mut false_body_context)?;
        }

        Ok(())
    }

    fn leave_if_statement<'a, 'b>(&mut self, context: &mut IfStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_if_statement(context)?;
        }

        Ok(())
    }

    fn visit_for_statement<'a, 'b>(&mut self, context: &mut ForStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_for_statement(context)?;
        }

        if let Some(statement) = context.for_statement.initialization_expression.as_ref() {
            let mut context = StatementContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement
            };

            self.visit_statement(&mut context)?;
            self.leave_statement(&mut context)?;
        }

        if let Some(expression) = context.for_statement.condition.as_ref() {
            let mut context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: None,
                expression
            };

            self.visit_expression(&mut context)?;
            self.leave_expression(&mut context)?;
        }

        if let Some(statement) = context.for_statement.loop_expression.as_ref() {
            let mut context = StatementContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement
            };

            self.visit_statement(&mut context)?;
            self.leave_statement(&mut context)?;
        }
        
        let mut context = BlockOrStatementContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            block_or_statement: &context.for_statement.body,
        };

        self.visit_block_or_statement(&mut context)?;
        self.leave_block_or_statement(&mut context)?;

        Ok(())
    }

    fn leave_for_statement<'a, 'b>(&mut self, context: &mut ForStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_for_statement(context)?;
        }

        Ok(())
    }

    fn visit_while_statement<'a, 'b>(&mut self, context: &mut WhileStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_while_statement(context)?;
        }

        let mut condition_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: None,
            expression: &context.while_statement.condition,
        };

        self.visit_expression(&mut condition_context)?;
        self.leave_expression(&mut condition_context)?;

        let mut context = BlockOrStatementContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            block_or_statement: &context.while_statement.body,
        };

        self.visit_block_or_statement(&mut context)?;
        self.leave_block_or_statement(&mut context)?;

        Ok(())
    }

    fn leave_while_statement<'a, 'b>(&mut self, context: &mut WhileStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_while_statement(context)?;
        }

        Ok(())
    }

    fn visit_emit_statement<'a, 'b>(&mut self, context: &mut EmitStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_emit_statement(context)?;
        }

        Ok(())
    }

    fn leave_emit_statement<'a, 'b>(&mut self, context: &mut EmitStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_emit_statement(context)?;
        }

        Ok(())
    }

    fn visit_try_statement<'a, 'b>(&mut self, context: &mut TryStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_try_statement(context)?;
        }

        for clause in context.try_statement.clauses.iter() {
            let mut context = BlockContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                block: &clause.block,
            };
            
            self.visit_block(&mut context)?;
            self.leave_block(&mut context)?;
        }

        Ok(())
    }

    fn leave_try_statement<'a, 'b>(&mut self, context: &mut TryStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_try_statement(context)?;
        }

        Ok(())
    }

    fn visit_revert_statement<'a, 'b>(&mut self, context: &mut RevertStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_revert_statement(context)?;
        }

        Ok(())
    }

    fn leave_revert_statement<'a, 'b>(&mut self, context: &mut RevertStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_revert_statement(context)?;
        }

        Ok(())
    }

    fn visit_block_or_statement<'a, 'b>(&mut self, context: &mut BlockOrStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_block_or_statement(context)?;
        }

        match context.block_or_statement {
            BlockOrStatement::Block(block) => {
                let mut context = BlockContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    block
                };
                
                self.visit_block(&mut context)?;
                self.leave_block(&mut context)?;
            }

            BlockOrStatement::Statement(statement) => {
                let mut context = StatementContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement
                };
    
                self.visit_statement(&mut context)?;
                self.leave_statement(&mut context)?;
            }
        }

        Ok(())
    }

    fn leave_block_or_statement<'a, 'b>(&mut self, context: &mut BlockOrStatementContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_block_or_statement(context)?;
        }

        Ok(())
    }

    fn visit_return<'a, 'b>(&mut self, context: &mut ReturnContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_return(context)?;
        }

        if let Some(expression) = context.return_statement.expression.as_ref() {
            let mut condition_context = ExpressionContext {
                source_units: context.source_units,
                current_source_unit: context.current_source_unit,
                contract_definition: context.contract_definition,
                definition_node: context.definition_node,
                blocks: context.blocks,
                statement: context.statement,
                expression
            };
    
            self.visit_expression(&mut condition_context)?;
            self.leave_expression(&mut condition_context)?;
        }

        Ok(())
    }

    fn visit_expression<'a, 'b>(&mut self, context: &mut ExpressionContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_expression(context)?;
        }

        match context.expression {
            Expression::Literal(literal) => {
                let mut context = LiteralContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    literal,
                };

                self.visit_literal(&mut context)?;
                self.leave_literal(&mut context)?;
            }

            Expression::Identifier(identifier) => {
                let mut context = IdentifierContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    identifier,
                };

                self.visit_identifier(&mut context)?;
                self.leave_identifier(&mut context)?;
            }

            Expression::UnaryOperation(unary_operation) => {
                let mut context = UnaryOperationContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    unary_operation,
                };

                self.visit_unary_operation(&mut context)?;
                self.leave_unary_operation(&mut context)?;
            }

            Expression::BinaryOperation(binary_operation) => {
                let mut context = BinaryOperationContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    binary_operation,
                };

                self.visit_binary_operation(&mut context)?;
                self.leave_binary_operation(&mut context)?;
            }

            Expression::Conditional(conditional) => {
                let mut context = ConditionalContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    conditional,
                };

                self.visit_conditional(&mut context)?;
                self.leave_conditional(&mut context)?;
            }

            Expression::Assignment(assignment) => {
                let mut context = AssignmentContext {
                    source_units: context.source_units,
                    current_source_unit: context.current_source_unit,
                    contract_definition: context.contract_definition,
                    definition_node: context.definition_node,
                    blocks: context.blocks,
                    statement: context.statement,
                    assignment,
                };

                self.visit_assignment(&mut context)?;
                self.leave_assignment(&mut context)?;
            }

            Expression::FunctionCall(function_call) => {
                self.visit_function_call(
                    context.current_source_unit,
                    context.contract_definition,
                    context.definition_node,
                    context.blocks,
                    context.statement,
                    function_call,
                )?;
            }

            Expression::FunctionCallOptions(function_call_options) => {
                self.visit_function_call_options(
                    context.current_source_unit,
                    context.contract_definition,
                    context.definition_node,
                    context.blocks,
                    context.statement,
                    function_call_options,
                )?;
            }

            Expression::IndexAccess(index_access) => {
                self.visit_index_access(
                    context.current_source_unit,
                    context.contract_definition,
                    context.definition_node,
                    context.blocks,
                    context.statement,
                    index_access,
                )?;
            }

            Expression::IndexRangeAccess(index_range_access) => {
                self.visit_index_range_access(
                    context.current_source_unit,
                    context.contract_definition,
                    context.definition_node,
                    context.blocks,
                    context.statement,
                    index_range_access,
                )?;
            }

            Expression::MemberAccess(member_access) => {
                self.visit_member_access(
                    context.current_source_unit,
                    context.contract_definition,
                    context.definition_node,
                    context.blocks,
                    context.statement,
                    member_access,
                )?;
            }

            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                self.visit_elementary_type_name_expression(
                    context.current_source_unit,
                    context.contract_definition,
                    context.definition_node,
                    context.blocks,
                    context.statement,
                    elementary_type_name_expression,
                )?;
            }

            Expression::TupleExpression(tuple_expression) => {
                self.visit_tuple_expression(
                    context.current_source_unit,
                    context.contract_definition,
                    context.definition_node,
                    context.blocks,
                    context.statement,
                    tuple_expression,
                )?;
            }

            Expression::NewExpression(new_expression) => {
                self.visit_new_expression(
                    context.current_source_unit,
                    context.contract_definition,
                    context.definition_node,
                    context.blocks,
                    context.statement,
                    new_expression,
                )?;
            }

            Expression::UnhandledExpression { node_type, src, id } => {
                self.visit_unhandled_expression(context.current_source_unit, node_type, src, id)?;
            }
        }

        Ok(())
    }

    fn visit_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_literal(context)?;
        }

        Ok(())
    }

    fn leave_literal<'a, 'b>(&mut self, context: &mut LiteralContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_literal(context)?;
        }

        Ok(())
    }

    fn visit_identifier<'a, 'b>(&mut self, context: &mut IdentifierContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_identifier(context)?;
        }

        Ok(())
    }

    fn leave_identifier<'a, 'b>(&mut self, context: &mut IdentifierContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_identifier(context)?;
        }

        Ok(())
    }

    fn visit_unary_operation<'a, 'b>(&mut self, context: &mut UnaryOperationContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_unary_operation(context)?;
        }

        let mut sub_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.unary_operation.sub_expression.as_ref(),
        };

        self.visit_expression(&mut sub_context)?;
        self.leave_expression(&mut sub_context)?;

        Ok(())
    }

    fn leave_unary_operation<'a, 'b>(&mut self, context: &mut UnaryOperationContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_unary_operation(context)?;
        }

        Ok(())
    }

    fn visit_binary_operation<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_binary_operation(context)?;
        }

        let mut left_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.binary_operation.left_expression.as_ref(),
        };

        self.visit_expression(&mut left_context)?;
        self.leave_expression(&mut left_context)?;

        let mut right_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.binary_operation.right_expression.as_ref(),
        };

        self.visit_expression(&mut right_context)?;
        self.leave_expression(&mut right_context)?;

        Ok(())
    }

    fn leave_binary_operation<'a, 'b>(&mut self, context: &mut BinaryOperationContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_binary_operation(context)?;
        }

        Ok(())
    }

    fn visit_conditional<'a, 'b>(&mut self, context: &mut ConditionalContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_conditional(context)?;
        }

        let mut condition_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.conditional.condition.as_ref(),
        };

        self.visit_expression(&mut condition_context)?;
        self.leave_expression(&mut condition_context)?;

        let mut true_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.conditional.true_expression.as_ref(),
        };

        self.visit_expression(&mut true_context)?;
        self.leave_expression(&mut true_context)?;

        let mut false_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.conditional.false_expression.as_ref(),
        };

        self.visit_expression(&mut false_context)?;
        self.leave_expression(&mut false_context)?;

        Ok(())
    }

    fn leave_conditional<'a, 'b>(&mut self, context: &mut ConditionalContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_conditional(context)?;
        }

        Ok(())
    }

    fn visit_assignment<'a, 'b>(&mut self, context: &mut AssignmentContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_assignment(context)?;
        }

        let mut left_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.assignment.left_hand_side.as_ref(),
        };

        self.visit_expression(&mut left_context)?;
        self.leave_expression(&mut left_context)?;

        let mut right_context = ExpressionContext {
            source_units: context.source_units,
            current_source_unit: context.current_source_unit,
            contract_definition: context.contract_definition,
            definition_node: context.definition_node,
            blocks: context.blocks,
            statement: context.statement,
            expression: context.assignment.right_hand_side.as_ref(),
        };

        self.visit_expression(&mut right_context)?;
        self.leave_expression(&mut right_context)?;

        Ok(())
    }

    fn leave_assignment<'a, 'b>(&mut self, context: &mut AssignmentContext<'a, 'b>) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.leave_assignment(context)?;
        }

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_function_call(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                function_call,
            )?;
        }

        let mut expression_context = ExpressionContext {
            source_units: &[], // TODO
            current_source_unit: source_unit,
            contract_definition: contract_definition,
            definition_node: definition_node,
            blocks: blocks,
            statement: statement,
            expression: function_call.expression.as_ref(),
        };

        self.visit_expression(&mut expression_context)?;
        self.leave_expression(&mut expression_context)?;

        for argument in function_call.arguments.iter() {
            let mut argument_context = ExpressionContext {
                source_units: &[], // TODO
                current_source_unit: source_unit,
                contract_definition: contract_definition,
                definition_node: definition_node,
                blocks: blocks,
                statement: statement,
                expression: argument,
            };
    
            self.visit_expression(&mut argument_context)?;
            self.leave_expression(&mut argument_context)?;
        }

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_function_call_options(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                function_call_options,
            )?;
        }

        let mut expression_context = ExpressionContext {
            source_units: &[], // TODO
            current_source_unit: source_unit,
            contract_definition: contract_definition,
            definition_node: definition_node,
            blocks: blocks,
            statement: statement,
            expression: function_call_options.expression.as_ref(),
        };

        self.visit_expression(&mut expression_context)?;
        self.leave_expression(&mut expression_context)?;

        for option in function_call_options.options.iter() {
            let mut option_context = ExpressionContext {
                source_units: &[], // TODO
                current_source_unit: source_unit,
                contract_definition: contract_definition,
                definition_node: definition_node,
                blocks: blocks,
                statement: statement,
                expression: option
            };
    
            self.visit_expression(&mut option_context)?;
            self.leave_expression(&mut option_context)?;
        }

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_index_access(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                index_access,
            )?;
        }

        let mut base_context = ExpressionContext {
            source_units: &[], // TODO
            current_source_unit: source_unit,
            contract_definition: contract_definition,
            definition_node: definition_node,
            blocks: blocks,
            statement: statement,
            expression: index_access.base_expression.as_ref(),
        };

        self.visit_expression(&mut base_context)?;
        self.leave_expression(&mut base_context)?;

        let mut index_context = ExpressionContext {
            source_units: &[], // TODO
            current_source_unit: source_unit,
            contract_definition: contract_definition,
            definition_node: definition_node,
            blocks: blocks,
            statement: statement,
            expression: index_access.index_expression.as_ref(),
        };

        self.visit_expression(&mut index_context)?;
        self.leave_expression(&mut index_context)?;

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_index_range_access(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                index_range_access,
            )?;
        }

        let mut base_context = ExpressionContext {
            source_units: &[], // TODO
            current_source_unit: source_unit,
            contract_definition: contract_definition,
            definition_node: definition_node,
            blocks: blocks,
            statement: statement,
            expression: index_range_access.base_expression.as_ref(),
        };

        self.visit_expression(&mut base_context)?;
        self.leave_expression(&mut base_context)?;

        if let Some(start_expression) = index_range_access.start_expression.as_ref() {
            let mut start_context = ExpressionContext {
                source_units: &[], // TODO
                current_source_unit: source_unit,
                contract_definition: contract_definition,
                definition_node: definition_node,
                blocks: blocks,
                statement: statement,
                expression: start_expression.as_ref(),
            };

            self.visit_expression(&mut start_context)?;
            self.leave_expression(&mut start_context)?;
        }

        if let Some(end_expression) = index_range_access.end_expression.as_ref() {
            let mut end_context = ExpressionContext {
                source_units: &[], // TODO
                current_source_unit: source_unit,
                contract_definition: contract_definition,
                definition_node: definition_node,
                blocks: blocks,
                statement: statement,
                expression: end_expression.as_ref(),
            };
    
            self.visit_expression(&mut end_context)?;
            self.leave_expression(&mut end_context)?;
        }

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_member_access(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                member_access,
            )?;
        }

        let mut expression_context = ExpressionContext {
            source_units: &[], // TODO
            current_source_unit: source_unit,
            contract_definition: contract_definition,
            definition_node: definition_node,
            blocks: blocks,
            statement: statement,
            expression: member_access.expression.as_ref(),
        };

        self.visit_expression(&mut expression_context)?;
        self.leave_expression(&mut expression_context)?;

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_elementary_type_name_expression(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                elementary_type_name_expression,
            )?;
        }

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_tuple_expression(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                tuple_expression,
            )?;
        }

        for component in tuple_expression.components.iter() {
            if let Some(component) = component {
                let mut component_context = ExpressionContext {
                    source_units: &[], // TODO
                    current_source_unit: source_unit,
                    contract_definition: contract_definition,
                    definition_node: definition_node,
                    blocks: blocks,
                    statement: statement,
                    expression: component,
                };
        
                self.visit_expression(&mut component_context)?;
                self.leave_expression(&mut component_context)?;
            }
        }

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_new_expression(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                new_expression,
            )?;
        }

        Ok(())
    }

    fn visit_unhandled_statement(
        &mut self,
        source_unit: &SourceUnit,
        node_type: &NodeType,
        src: &Option<String>,
        id: &Option<NodeID>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_unhandled_statement(source_unit, node_type, src, id)?;
        }

        match node_type {
            NodeType::Break | NodeType::Continue | NodeType::PlaceholderStatement => Ok(()),

            _ => {
                println!(
                    "WARNING: Unhandled statement: {:?} {:?} {:?}",
                    node_type, src, id
                );
                Ok(())
            }
        }
    }

    fn visit_unhandled_expression(
        &mut self,
        source_unit: &SourceUnit,
        node_type: &NodeType,
        src: &Option<String>,
        id: &Option<NodeID>,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_unhandled_expression(source_unit, node_type, src, id)?;
        }

        match node_type {
            NodeType::PlaceholderStatement => Ok(()),

            _ => {
                println!(
                    "WARNING: Unhandled expression: {:?} {:?} {:?}",
                    node_type, src, id
                );
                Ok(())
            }
        }
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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_inline_assembly(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
            )?;
        }

        if let Some(yul_block) = inline_assembly.ast.as_ref() {
            let mut yul_blocks = vec![];

            self.visit_yul_block(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                &mut yul_blocks,
                yul_block,
            )?;
        }

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
        yul_blocks.push(yul_block);

        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_block(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_block,
            )?;
        }

        for yul_statement in yul_block.statements.iter() {
            self.visit_yul_statement(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
            )?;
        }

        yul_blocks.pop();

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_statement(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
            )?;
        }

        match yul_statement {
            YulStatement::YulIf(yul_if) => {
                self.visit_yul_if(
                    source_unit,
                    contract_definition,
                    definition_node,
                    blocks,
                    statement,
                    inline_assembly,
                    yul_blocks,
                    yul_statement,
                    yul_if,
                )?;
            }

            YulStatement::YulSwitch(yul_switch) => {
                self.visit_yul_switch(
                    source_unit,
                    contract_definition,
                    definition_node,
                    blocks,
                    statement,
                    inline_assembly,
                    yul_blocks,
                    yul_statement,
                    yul_switch,
                )?;
            }

            YulStatement::YulAssignment(yul_assignment) => {
                self.visit_yul_assignment(
                    source_unit,
                    contract_definition,
                    definition_node,
                    blocks,
                    statement,
                    inline_assembly,
                    yul_blocks,
                    yul_statement,
                    yul_assignment,
                )?;
            }

            YulStatement::YulVariableDeclaration(yul_variable_declaration) => {
                self.visit_yul_variable_declaration(
                    source_unit,
                    contract_definition,
                    definition_node,
                    blocks,
                    statement,
                    inline_assembly,
                    yul_blocks,
                    yul_statement,
                    yul_variable_declaration,
                )?;
            }

            YulStatement::YulExpressionStatement(yul_expression_statement) => {
                self.visit_yul_expression_statement(
                    source_unit,
                    contract_definition,
                    definition_node,
                    blocks,
                    statement,
                    inline_assembly,
                    yul_blocks,
                    yul_statement,
                    yul_expression_statement,
                )?;
            }

            YulStatement::UnhandledYulStatement { node_type, src, id } => {
                println!(
                    "WARNING: Unhandled yul statement: {:?} {:?} {:?}",
                    node_type, src, id
                );
            }
        }

        Ok(())
    }

    fn visit_yul_if<'a>(
        &mut self,
        source_unit: &'a SourceUnit,
        contract_definition: &'a ContractDefinition,
        definition_node: &'a ContractDefinitionNode,
        blocks: &mut Vec<&'a Block>,
        statement: &'a Statement,
        inline_assembly: &'a InlineAssembly,
        yul_blocks: &mut Vec<&'a YulBlock>,
        yul_statement: &'a YulStatement,
        yul_if: &'a YulIf,
    ) -> io::Result<()> {
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_if(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_if,
            )?;
        }

        self.visit_yul_expression(
            source_unit,
            contract_definition,
            definition_node,
            blocks,
            statement,
            inline_assembly,
            yul_blocks,
            Some(yul_statement),
            &yul_if.condition,
        )?;

        self.visit_yul_block(
            source_unit,
            contract_definition,
            definition_node,
            blocks,
            statement,
            inline_assembly,
            yul_blocks,
            &yul_if.body,
        )?;

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_switch(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_switch,
            )?;
        }

        self.visit_yul_expression(
            source_unit,
            contract_definition,
            definition_node,
            blocks,
            statement,
            inline_assembly,
            yul_blocks,
            Some(yul_statement),
            &yul_switch.expression,
        )?;

        for yul_case in yul_switch.cases.iter() {
            self.visit_yul_case(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_switch,
                yul_case,
            )?;
        }

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_case(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_switch,
                yul_case,
            )?;
        }

        self.visit_yul_expression(
            source_unit,
            contract_definition,
            definition_node,
            blocks,
            statement,
            inline_assembly,
            yul_blocks,
            Some(yul_statement),
            &yul_case.value,
        )?;

        self.visit_yul_block(
            source_unit,
            contract_definition,
            definition_node,
            blocks,
            statement,
            inline_assembly,
            yul_blocks,
            &yul_case.body,
        )?;

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_assignment(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_assignment,
            )?;
        }

        for yul_identifier in yul_assignment.variable_names.iter() {
            self.visit_yul_identifier(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                Some(yul_statement),
                None,
                yul_identifier,
            )?;
        }

        self.visit_yul_expression(
            source_unit,
            contract_definition,
            definition_node,
            blocks,
            statement,
            inline_assembly,
            yul_blocks,
            Some(yul_statement),
            &yul_assignment.value,
        )?;

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_variable_declaration(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_variable_declaration,
            )?;
        }

        self.visit_yul_expression(
            source_unit,
            contract_definition,
            definition_node,
            blocks,
            statement,
            inline_assembly,
            yul_blocks,
            Some(yul_statement),
            &yul_variable_declaration.value,
        )?;

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_expression_statement(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_expression_statement,
            )?;
        }

        self.visit_yul_expression(
            source_unit,
            contract_definition,
            definition_node,
            blocks,
            statement,
            inline_assembly,
            yul_blocks,
            Some(yul_statement),
            &yul_expression_statement.expression,
        )?;

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_expression(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_expression,
            )?;
        }

        match yul_expression {
            YulExpression::YulLiteral(yul_literal) => {
                self.visit_yul_literal(
                    source_unit,
                    contract_definition,
                    definition_node,
                    blocks,
                    statement,
                    inline_assembly,
                    yul_blocks,
                    yul_statement,
                    yul_expression,
                    yul_literal,
                )?;
            }

            YulExpression::YulIdentifier(yul_identifier) => {
                self.visit_yul_identifier(
                    source_unit,
                    contract_definition,
                    definition_node,
                    blocks,
                    statement,
                    inline_assembly,
                    yul_blocks,
                    yul_statement,
                    Some(yul_expression),
                    yul_identifier,
                )?;
            }

            YulExpression::YulFunctionCall(yul_function_call) => {
                self.visit_yul_function_call(
                    source_unit,
                    contract_definition,
                    definition_node,
                    blocks,
                    statement,
                    inline_assembly,
                    yul_blocks,
                    yul_statement,
                    yul_expression,
                    yul_function_call,
                )?;
            }

            YulExpression::UnhandledYulExpression { node_type, src, id } => {
                println!(
                    "WARNING: Unhandled yul expression: {:?} {:?} {:?}",
                    node_type, src, id
                );
            }
        }

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_literal(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_expression,
                yul_literal,
            )?;
        }

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_identifier(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_expression,
                yul_identifier,
            )?;
        }

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
        for visitor in self.visitors.iter_mut() {
            visitor.visit_yul_function_call(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_expression,
                yul_function_call,
            )?;
        }

        self.visit_yul_identifier(
            source_unit,
            contract_definition,
            definition_node,
            blocks,
            statement,
            inline_assembly,
            yul_blocks,
            yul_statement,
            Some(yul_expression),
            &yul_function_call.function_name,
        )?;

        for yul_expression in yul_function_call.arguments.iter() {
            self.visit_yul_expression(
                source_unit,
                contract_definition,
                definition_node,
                blocks,
                statement,
                inline_assembly,
                yul_blocks,
                yul_statement,
                yul_expression,
            )?;
        }

        Ok(())
    }
}

pub fn visit_source_units<'a>(visitors: Vec<Box<dyn AstVisitor + 'a>>, source_units: &[SourceUnit]) -> io::Result<()> {
    let mut data = AstVisitorData {
        analyzed_paths: HashSet::new(),
        visitors
    };

    for source_unit in source_units.iter() {
        if let Some(path) = source_unit.absolute_path.as_ref() {
            if data.analyzed_paths.contains(path) {
                return Ok(());
            }

            data.analyzed_paths.insert(path.clone());
        }

        let mut context = SourceUnitContext {
            source_units,
            current_source_unit: source_unit
        };

        data.visit_source_unit(&mut context)?;
        data.leave_source_unit(&mut context)?;
    }

    Ok(())
}
