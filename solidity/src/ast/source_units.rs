use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PragmaDirective {
    pub literals: Vec<String>,
    pub src: String,
    pub id: NodeID,
}

pub struct PragmaDirectiveContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub pragma_directive: &'a PragmaDirective,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SymbolAlias {
    pub foreign: Expression,
    pub local: Option<String>,
    pub name_location: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImportDirective {
    pub file: String,
    pub source_unit: NodeID,
    pub scope: NodeID,
    pub absolute_path: Option<String>,
    pub unit_alias: String,
    pub name_location: Option<String>,
    pub symbol_aliases: Vec<SymbolAlias>,
    pub src: String,
    pub id: NodeID,
}

pub struct ImportDirectiveContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
    pub import_directive: &'a ImportDirective,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SourceUnitNode {
    PragmaDirective(PragmaDirective),
    ImportDirective(ImportDirective),
    ContractDefinition(ContractDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceUnit {
    pub license: Option<String>,
    pub nodes: Vec<SourceUnitNode>,
    pub exported_symbols: Option<HashMap<String, Vec<NodeID>>>,
    pub absolute_path: Option<String>,
    pub id: NodeID,

    #[serde(skip_serializing)]
    pub source: Option<String>,
}

impl SourceUnit {
    pub fn source_line(&self, src: &str) -> Option<usize> {
        let source = match self.source.as_ref() {
            Some(source) => source.as_str(),
            None => return None,
        };

        let mut tokens = src.split(':');
        let mut values: Vec<Option<usize>> = vec![];

        while let Some(token) = tokens.next() {
            values.push(if token.is_empty() {
                None
            } else {
                Some(match token.parse() {
                    Ok(value) => value,
                    Err(error) => {
                        println!("ERROR: {}: {}", error, token);
                        return None;
                    }
                })
            });
        }

        Some(
            source[..match values.first() {
                Some(&Some(value)) => value,
                _ => return None,
            }]
                .chars()
                .filter(|&c| c == '\n')
                .count()
                + 1,
        )
    }

    pub fn pragma_directives(&self) -> Vec<&PragmaDirective> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let SourceUnitNode::PragmaDirective(pragma_directive) = node {
                result.push(pragma_directive);
            }
        }

        result
    }

    pub fn import_directives(&self) -> Vec<&ImportDirective> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let SourceUnitNode::ImportDirective(import_directive) = node {
                result.push(import_directive);
            }
        }

        result
    }

    pub fn contract_definitions(&self) -> Vec<&ContractDefinition> {
        let mut result = vec![];

        for node in self.nodes.iter() {
            if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                result.push(contract_definition);
            }
        }

        result
    }

    pub fn contract_definition(&self, id: NodeID) -> Option<&ContractDefinition> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                if id == contract_definition.id {
                    return Some(contract_definition);
                }
            }
        }

        None
    }

    pub fn struct_definition(&self, id: NodeID) -> Option<&StructDefinition> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::StructDefinition(struct_definition) = node {
                if id == struct_definition.id {
                    return Some(struct_definition);
                }
            }
        }

        None
    }

    pub fn enum_definition(&self, id: NodeID) -> Option<&EnumDefinition> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::EnumDefinition(enum_definition) = node {
                if id == enum_definition.id {
                    return Some(enum_definition);
                }
            }
        }

        None
    }

    pub fn function_definition(&self, id: NodeID) -> Option<&FunctionDefinition> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                for node in contract_definition.nodes.iter() {
                    if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                        if function_definition.id == id {
                            return Some(function_definition);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn function_and_contract_definition(
        &self,
        id: NodeID,
    ) -> Option<(&ContractDefinition, &FunctionDefinition)> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                for node in contract_definition.nodes.iter() {
                    if let ContractDefinitionNode::FunctionDefinition(function_definition) = node {
                        if function_definition.id == id {
                            return Some((contract_definition, function_definition));
                        }
                    }
                }
            }
        }

        None
    }

    pub fn find_contract_definition_node(
        &self,
        id: NodeID,
    ) -> Option<(&ContractDefinition, &ContractDefinitionNode)> {
        for node in self.nodes.iter() {
            if let SourceUnitNode::ContractDefinition(contract_definition) = node {
                for node in contract_definition.nodes.iter() {
                    if id
                        == match node {
                            ContractDefinitionNode::UsingForDirective(node) => node.id,
                            ContractDefinitionNode::StructDefinition(node) => node.id,
                            ContractDefinitionNode::EnumDefinition(node) => node.id,
                            ContractDefinitionNode::VariableDeclaration(node) => node.id,
                            ContractDefinitionNode::EventDefinition(node) => node.id,
                            ContractDefinitionNode::FunctionDefinition(node) => node.id,
                            ContractDefinitionNode::ModifierDefinition(node) => node.id,
                            ContractDefinitionNode::ErrorDefinition(node) => node.id,
                        }
                    {
                        return Some((contract_definition, node));
                    }
                }
            }
        }

        None
    }
}

pub struct SourceUnitContext<'a> {
    pub source_units: &'a [SourceUnit],
    pub current_source_unit: &'a SourceUnit,
}

impl<'a> SourceUnitContext<'a> {
    pub fn create_pragma_directive_context(
        &self,
        pragma_directive: &'a PragmaDirective,
    ) -> PragmaDirectiveContext<'a> {
        PragmaDirectiveContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            pragma_directive,
        }
    }

    pub fn create_import_directive_context(
        &self,
        import_directive: &'a ImportDirective,
    ) -> ImportDirectiveContext<'a> {
        ImportDirectiveContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            import_directive,
        }
    }

    pub fn create_contract_definition_context(
        &self,
        contract_definition: &'a ContractDefinition,
    ) -> ContractDefinitionContext<'a> {
        ContractDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition,
        }
    }

    pub fn create_struct_definition_context(
        &self,
        struct_definition: &'a StructDefinition,
    ) -> StructDefinitionContext<'a> {
        StructDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: None,
            struct_definition,
        }
    }

    pub fn create_enum_definition_context(
        &self,
        enum_definition: &'a EnumDefinition,
    ) -> EnumDefinitionContext<'a> {
        EnumDefinitionContext {
            source_units: self.source_units,
            current_source_unit: self.current_source_unit,
            contract_definition: None,
            enum_definition,
        }
    }
}
