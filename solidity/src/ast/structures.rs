use crate::ast::{NodeID, VariableDeclaration, Visibility};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructDefinition {
    pub name: String,
    pub name_location: Option<String>,
    pub visibility: Visibility,
    pub members: Vec<VariableDeclaration>,
    pub scope: NodeID,
    pub canonical_name: Option<String>,
    pub src: String,
    pub id: NodeID,
}

impl Display for StructDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("struct {} {{\n", self.name))?;

        for member in self.members.iter() {
            f.write_fmt(format_args!("\t{};\n", member))?;
        }

        f.write_str("}")
    }
}
