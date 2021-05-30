use crate::ast::{
    Documentation, EnumDefinition, ErrorDefinition, EventDefinition, Expression,
    FunctionDefinition, IdentifierPath, ModifierDefinition, NodeID, StructDefinition, TypeName,
    VariableDeclaration,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
    Contract,
    Interface,
    Library,
}

impl Display for ContractKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", format!("{:?}", self).to_lowercase()))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ContractDefinitionNode {
    UsingForDirective(UsingForDirective),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    VariableDeclaration(VariableDeclaration),
    EventDefinition(EventDefinition),
    FunctionDefinition(FunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    ErrorDefinition(ErrorDefinition),
}

impl Display for ContractDefinitionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractDefinitionNode::UsingForDirective(using_for_directive) => {
                using_for_directive.fmt(f)
            }
            ContractDefinitionNode::StructDefinition(struct_definition) => struct_definition.fmt(f),
            ContractDefinitionNode::EnumDefinition(enum_definition) => enum_definition.fmt(f),
            ContractDefinitionNode::VariableDeclaration(variable_declaration) => {
                variable_declaration.fmt(f)
            }
            ContractDefinitionNode::EventDefinition(event_definition) => event_definition.fmt(f),
            ContractDefinitionNode::FunctionDefinition(function_definition) => {
                function_definition.fmt(f)
            }
            ContractDefinitionNode::ModifierDefinition(modifier_definition) => {
                modifier_definition.fmt(f)
            }
            ContractDefinitionNode::ErrorDefinition(error_definition) => error_definition.fmt(f),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InheritanceSpecifier {
    pub arguments: Option<Vec<Expression>>,
    pub base_name: IdentifierPath,
    pub src: String,
    pub id: NodeID,
}

impl Display for InheritanceSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.base_name))?;

        if let Some(arguments) = self.arguments.as_ref() {
            f.write_str("(")?;

            for (i, argument) in arguments.iter().enumerate() {
                f.write_fmt(format_args!(
                    "{}{}",
                    match i {
                        0 => "",
                        _ => ", ",
                    },
                    argument,
                ))?;
            }

            f.write_str(")")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OverrideSpecifier {
    pub overrides: Vec<IdentifierPath>,
    pub src: String,
    pub id: NodeID,
}

impl Display for OverrideSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("override")?;

        if !self.overrides.is_empty() {
            f.write_str("(")?;

            for (i, identifier_path) in self.overrides.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }

                f.write_fmt(format_args!("{}", identifier_path))?;
            }

            f.write_str(")")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsingForDirective {
    pub library_name: IdentifierPath,
    pub type_name: Option<TypeName>,
    pub src: String,
    pub id: NodeID,
}

impl Display for UsingForDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "using {} for {}",
            self.library_name,
            match self.type_name.as_ref() {
                Some(type_name) => format!("{}", type_name),
                None => format!("_"),
            }
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    #[serde(rename = "abstract")]
    pub is_abstract: Option<bool>,
    pub base_contracts: Vec<InheritanceSpecifier>,
    pub contract_dependencies: Vec<NodeID>,
    #[serde(rename = "contractKind")]
    pub kind: ContractKind,
    pub documentation: Option<Documentation>,
    pub fully_implemented: bool,
    pub linearized_base_contracts: Vec<NodeID>,
    pub used_errors: Option<Vec<NodeID>>,
    pub name: String,
    pub name_location: Option<String>,
    pub nodes: Vec<ContractDefinitionNode>,
    pub scope: NodeID,
    pub src: String,
    pub id: NodeID,
}

impl ContractDefinition {
    pub fn using_for_directive(&self, id: NodeID) -> Option<&UsingForDirective> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::UsingForDirective(using_for_directive) = node {
                if using_for_directive.id == id {
                    return Some(using_for_directive);
                }
            }
        }

        None
    }

    pub fn using_for_directives(&self) -> Vec<&UsingForDirective> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::UsingForDirective(using_for_directive) = node {
                result.push(using_for_directive);
            }
        }

        result
    }

    pub fn struct_definition(&self, id: NodeID) -> Option<&StructDefinition> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::StructDefinition(struct_definition) = node {
                if id == struct_definition.id {
                    return Some(struct_definition);
                }
            }
        }

        None
    }

    pub fn struct_definitions(&self) -> Vec<&StructDefinition> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::StructDefinition(struct_definition) = node {
                result.push(struct_definition);
            }
        }

        result
    }

    pub fn enum_definition(&self, id: NodeID) -> Option<&EnumDefinition> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::EnumDefinition(enum_definition) = node {
                if id == enum_definition.id {
                    return Some(enum_definition);
                }
            }
        }

        None
    }

    pub fn enum_definitions(&self) -> Vec<&EnumDefinition> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::EnumDefinition(enum_definition) = node {
                result.push(enum_definition);
            }
        }

        result
    }

    pub fn event_definition(&self, id: NodeID) -> Option<&EventDefinition> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::EventDefinition(event_definition) = node {
                if id == event_definition.id {
                    return Some(event_definition);
                }
            }
        }

        None
    }

    pub fn variable_declaration(&self, id: NodeID) -> Option<&VariableDeclaration> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::VariableDeclaration(variable_declaration) = node {
                if id == variable_declaration.id {
                    return Some(variable_declaration);
                }
            }
        }

        None
    }

    pub fn variable_declarations(&self) -> Vec<&VariableDeclaration> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::VariableDeclaration(variable_declaration) = node {
                result.push(variable_declaration);
            }
        }

        result
    }

    pub fn function_definition(&self, id: NodeID) -> Option<&FunctionDefinition> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                if id == function_definition.id {
                    return Some(function_definition);
                }
            }
        }

        None
    }

    pub fn function_definitions(&self) -> Vec<&FunctionDefinition> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                result.push(function_definition);
            }
        }

        result
    }

    pub fn modifier_definition(&self, id: NodeID) -> Option<&ModifierDefinition> {
        for node in self.nodes.iter() {
            if let ContractDefinitionNode::ModifierDefinition(modifier_definition) = node {
                if id == modifier_definition.id {
                    return Some(modifier_definition);
                }
            }
        }

        None
    }

    pub fn modifier_definitions(&self) -> Vec<&ModifierDefinition> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let ContractDefinitionNode::ModifierDefinition(modifier_definition) = node {
                result.push(modifier_definition);
            }
        }

        result
    }
}

impl Display for ContractDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(true) = self.is_abstract {
            f.write_str("abstract ")?;
        }

        f.write_fmt(format_args!("{} {}", self.kind, self.name))?;

        for (i, base_contract) in self.base_contracts.iter().enumerate() {
            f.write_fmt(format_args!(
                "{}{}",
                match i {
                    0 => " is ",
                    _ => ", ",
                },
                base_contract
            ))?;
        }

        f.write_str(" {\n")?;

        for node in self.nodes.iter() {
            f.write_fmt(format_args!(
                "\t{}{}\n",
                node,
                match node {
                    ContractDefinitionNode::UsingForDirective(_)
                    | ContractDefinitionNode::EventDefinition(_)
                    | ContractDefinitionNode::ErrorDefinition(_)
                    | ContractDefinitionNode::VariableDeclaration(_) => ";",

                    ContractDefinitionNode::StructDefinition(_)
                    | ContractDefinitionNode::EnumDefinition(_)
                    | ContractDefinitionNode::FunctionDefinition(_)
                    | ContractDefinitionNode::ModifierDefinition(_) => "",
                }
            ))?;
        }

        f.write_str("}")?;

        Ok(())
    }
}