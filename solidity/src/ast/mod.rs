mod blocks;
mod contracts;
mod documentation;
mod enumerations;
mod errors;
mod events;
mod expressions;
mod functions;
mod identifiers;
mod import_directives;
mod literals;
mod modifiers;
mod pragma_directives;
mod source_units;
mod statements;
mod structures;
mod types;
mod using_for_directives;
mod variables;
mod visitor;

pub use self::{
    blocks::*, contracts::*, documentation::*, enumerations::*, errors::*, events::*,
    expressions::*, functions::*, identifiers::*, import_directives::*, literals::*, modifiers::*,
    pragma_directives::*, source_units::*, statements::*, structures::*, types::*,
    using_for_directives::*, variables::*, visitor::*,
};

use serde::{Deserialize, Serialize};

pub type NodeID = i64;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum NodeType {
    SourceUnit,
    PragmaDirective,
    ImportDirective,
    UsingForDirective,
    ContractDefinition,
    InheritanceSpecifier,
    OverrideSpecifier,
    IdentifierPath,
    StructuredDocumentation,
    VariableDeclaration,
    Mapping,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    ArrayTypeName,
    TupleExpression,
    FunctionDefinition,
    ParameterList,
    Block,
    UncheckedBlock,
    Continue,
    Break,
    Return,
    Throw,
    Literal,
    Conditional,
    Identifier,
    IndexAccess,
    IndexRangeAccess,
    MemberAccess,
    Assignment,
    FunctionCall,
    FunctionCallOptions,
    FunctionTypeName,
    NewExpression,
    ExpressionStatement,
    VariableDeclarationStatement,
    IfStatement,
    TryCatchClause,
    UnaryOperation,
    BinaryOperation,
    EventDefinition,
    ErrorDefiniton,
    EmitStatement,
    PlaceholderStatement,
    TryStatement,
    RevertStatement,
    ForStatement,
    WhileStatement,
    ModifierDefinition,
    ModifierInvocation,
    EnumDefinition,
    EnumValue,
    StructDefinition,
    UserDefinedTypeName,
    InlineAssembly,
    YulLiteral,
    YulTypedName,
    YulSwitch,
    YulCase,
    YulFunctionCall,
    YulExpressionStatement,
    YulAssignment,
    YulIdentifier,
    YulVariableDeclaration,
    YulBlock,
}
