use crate::ast::{Identifier, Literal, NodeID, NodeType, TypeDescriptions, TypeName};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    Conditional(Conditional),
    Assignment(Assignment),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    MemberAccess(MemberAccess),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    TupleExpression(TupleExpression),
    NewExpression(NewExpression),

    #[serde(rename_all = "camelCase")]
    UnhandledExpression {
        node_type: NodeType,
        src: Option<String>,
        id: Option<NodeID>,
    },
}

impl Expression {
    pub fn root_expression(&self) -> Option<&Expression> {
        match self {
            Expression::Identifier(_) => Some(self),
            Expression::TupleExpression(_) => Some(self),
            Expression::Assignment(assignment) => assignment.left_hand_side.root_expression(),
            Expression::IndexAccess(index_access) => index_access.base_expression.root_expression(),
            Expression::IndexRangeAccess(index_range_access) => {
                index_range_access.base_expression.root_expression()
            }
            Expression::MemberAccess(member_access) => member_access.expression.root_expression(),
            _ => None,
        }
    }

    pub fn referenced_declarations(&self) -> Vec<NodeID> {
        let mut result = vec![];

        match self {
            Expression::Identifier(identifier) => {
                result.push(identifier.referenced_declaration);
            }

            Expression::Assignment(assignment) => {
                result.extend(assignment.left_hand_side.referenced_declarations());
                result.extend(assignment.right_hand_side.referenced_declarations());
            }

            Expression::IndexAccess(index_access) => {
                result.extend(index_access.base_expression.referenced_declarations());
            }

            Expression::IndexRangeAccess(index_range_access) => {
                result.extend(index_range_access.base_expression.referenced_declarations());
            }

            Expression::MemberAccess(member_access) => {
                result.extend(member_access.expression.referenced_declarations());

                if let Some(referenced_declaration) = member_access.referenced_declaration {
                    result.push(referenced_declaration);
                }
            }

            Expression::TupleExpression(tuple_expression) => {
                for component in tuple_expression.components.iter() {
                    if let Some(component) = component {
                        result.extend(component.referenced_declarations());
                    }
                }
            }

            _ => {}
        }

        result
    }

    pub fn contains_operation(&self, operator: &str) -> bool {
        match self {
            Expression::UnaryOperation(unary_operation) => unary_operation.contains_operation(operator),
            Expression::BinaryOperation(binary_operation) => binary_operation.contains_operation(operator),
            Expression::Conditional(conditional) => conditional.contains_operation(operator),
            Expression::Assignment(assignment) => assignment.contains_operation(operator),
            Expression::FunctionCall(function_call) => function_call.contains_operation(operator),
            Expression::FunctionCallOptions(function_call_options) => function_call_options.contains_operation(operator),
            Expression::IndexAccess(index_access) => index_access.contains_operation(operator),
            Expression::IndexRangeAccess(index_range_access) => index_range_access.contains_operation(operator),
            Expression::MemberAccess(member_access) => member_access.contains_operation(operator),
            Expression::TupleExpression(tuple_expression) => tuple_expression.contains_operation(operator),
            _ => false
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(expr) => expr.fmt(f)?,
            Expression::Identifier(expr) => expr.fmt(f)?,
            Expression::UnaryOperation(expr) => expr.fmt(f)?,
            Expression::BinaryOperation(expr) => expr.fmt(f)?,
            Expression::Conditional(expr) => expr.fmt(f)?,
            Expression::Assignment(expr) => expr.fmt(f)?,
            Expression::FunctionCall(expr) => expr.fmt(f)?,
            Expression::FunctionCallOptions(expr) => expr.fmt(f)?,
            Expression::IndexAccess(expr) => expr.fmt(f)?,
            Expression::IndexRangeAccess(expr) => expr.fmt(f)?,
            Expression::MemberAccess(expr) => expr.fmt(f)?,
            Expression::ElementaryTypeNameExpression(expr) => expr.fmt(f)?,
            Expression::TupleExpression(expr) => expr.fmt(f)?,
            Expression::NewExpression(expr) => expr.fmt(f)?,
            _ => {}
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnaryOperation {
    pub prefix: bool,
    pub sub_expression: Box<Expression>,
    pub operator: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
}

impl UnaryOperation {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator ||
        self.sub_expression.contains_operation(operator)
    }
}

impl Display for UnaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}",
            self.sub_expression,
            self.operator.as_str()
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryOperation {
    pub common_type: TypeDescriptions,
    pub left_expression: Box<Expression>,
    pub right_expression: Box<Expression>,
    pub operator: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl BinaryOperation {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator ||
        self.left_expression.contains_operation(operator) ||
        self.right_expression.contains_operation(operator)
    }
}

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            self.left_expression, self.operator, self.right_expression
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub true_expression: Box<Expression>,
    pub false_expression: Box<Expression>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl Conditional {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.condition.contains_operation(operator) ||
        self.true_expression.contains_operation(operator) ||
        self.false_expression.contains_operation(operator)
    }
}

impl Display for Conditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} ? {} : {}",
            self.condition, self.true_expression, self.false_expression
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub left_hand_side: Box<Expression>,
    pub right_hand_side: Box<Expression>,
    pub operator: String,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl Assignment {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator || self.right_hand_side.contains_operation(operator)
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            self.left_hand_side, self.operator, self.right_hand_side
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub enum FunctionCallKind {
    FunctionCall,
    TypeConversion,
    StructConstructorCall,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub kind: FunctionCallKind,
    pub try_call: Option<bool>,
    pub names: Vec<String>,
    pub arguments: Vec<Expression>,
    pub expression: Box<Expression>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl FunctionCall {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for argument in self.arguments.iter() {
            if argument.contains_operation(operator) {
                return true;
            }
        }

        false
    }
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.expression))?;
        f.write_str("(")?;

        for (i, argument) in self.arguments.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }

            f.write_fmt(format_args!("{}", argument))?;
        }

        f.write_str(")")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallOptions {
    pub names: Vec<String>,
    pub options: Vec<Expression>,
    pub argument_types: Vec<TypeDescriptions>,
    pub expression: Box<Expression>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
}

impl FunctionCallOptions {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for option in self.options.iter() {
            if option.contains_operation(operator) {
                return true;
            }
        }

        false
    }
}

impl Display for FunctionCallOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let option_count = self.options.len();
        
        if self.names.len() != option_count {
            eprintln!("ERROR: invalid FunctionCallOptions: {:?}, {:?}", self.names, self.options);
            return Err(std::fmt::Error)
        }

        f.write_fmt(format_args!("{}", self.expression))?;

        for i in 0..option_count {
            if i > 0 {
                f.write_str(", ")?;
            }

            f.write_fmt(format_args!("{}: {}", self.names[i], self.options[i]))?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexAccess {
    pub base_expression: Box<Expression>,
    pub index_expression: Box<Expression>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl IndexAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.index_expression.contains_operation(operator)
    }
}

impl Display for IndexAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}[{}]", self.base_expression, self.index_expression))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexRangeAccess {
    pub base_expression: Box<Expression>,
    pub start_expression: Option<Box<Expression>>,
    pub end_expression: Option<Box<Expression>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl IndexRangeAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.start_expression.as_ref().map(|expr| expr.contains_operation(operator)).unwrap_or(false) ||
        self.end_expression.as_ref().map(|expr| expr.contains_operation(operator)).unwrap_or(false)
    }
}

impl Display for IndexRangeAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}[", self.base_expression))?;

        if let Some(start_expression) = self.start_expression.as_ref() {
            f.write_fmt(format_args!("{}", start_expression))?;
        }

        f.write_str(":")?;

        if let Some(end_expression) = self.end_expression.as_ref() {
            f.write_fmt(format_args!("{}", end_expression))?;
        }

        f.write_str("]")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MemberAccess {
    pub member_name: String,
    pub expression: Box<Expression>,
    pub referenced_declaration: Option<NodeID>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl MemberAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.expression.contains_operation(operator)
    }
}

impl Display for MemberAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.expression, self.member_name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeNameExpression {
    pub type_name: TypeName,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl Display for ElementaryTypeNameExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.type_name))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TupleExpression {
    pub components: Vec<Option<Expression>>,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_inline_array: bool,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub src: String,
    pub id: NodeID,
}

impl TupleExpression {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for component in self.components.iter() {
            if component.as_ref().map(|expr| expr.contains_operation(operator)).unwrap_or(false) {
                return true;
            }
        }

        false
    }
}

impl Display for TupleExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;

        for (i, component) in self.components.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }

            if let Some(component) = component {
                f.write_fmt(format_args!("{}", component))?;
            }
        }

        f.write_str(")")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewExpression {
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub type_descriptions: TypeDescriptions,
    pub type_name: TypeName,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub src: String,
    pub id: NodeID,
}

impl Display for NewExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("new {}", self.type_name))
    }
}