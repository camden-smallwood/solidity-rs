use crate::ast::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModifierDefinition {
    pub body: Block,
    pub overrides: Option<OverrideSpecifier>,
    pub documentation: Option<Documentation>,
    pub name: String,
    pub name_location: Option<String>,
    pub parameters: ParameterList,
    pub r#virtual: Option<bool>,
    pub visibility: Visibility,
    pub src: String,
    pub id: NodeID,
}

impl Display for ModifierDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("modifier")?;

        if !self.name.is_empty() {
            f.write_fmt(format_args!(" {}", self.name))?;
        }

        f.write_fmt(format_args!("{}", self.parameters))?;
                
        if self.visibility != Visibility::Internal {
            f.write_fmt(format_args!("{} {}", self.parameters, self.visibility))?;
        }
        
        if let Some(true) = self.r#virtual {
            f.write_fmt(format_args!(" virtual"))?;
        }

        if let Some(overrides) = self.overrides.as_ref() {
            f.write_fmt(format_args!(" {}", overrides))?;
        }
        
        f.write_fmt(format_args!(" {}", self.body))
    }
}

pub struct ModifierDefinitionContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub modifier_definition: &'a ModifierDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ModifierInvocationKind {
    ModifierInvocation,
    BaseConstructorSpecifier,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModifierInvocation {
    pub arguments: Option<Vec<Expression>>,
    pub modifier_name: IdentifierPath,
    pub src: String,
    pub id: NodeID,
    pub kind: Option<ModifierInvocationKind>,
}

impl Display for ModifierInvocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.modifier_name))?;

        if let Some(arguments) = self.arguments.as_ref() {
            f.write_str("(")?;

            for (i, argument) in arguments.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }

                f.write_fmt(format_args!("{}", argument))?;
            }

            f.write_str(")")?;
        }

        Ok(())
    }
}

pub struct ModifierInvocationContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub contract_definition: &'a ContractDefinition,
    pub definition_node: &'a ContractDefinitionNode,
    pub modifier_invocation: &'a ModifierInvocation,
}
