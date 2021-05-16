use crate::ast::{Documentation, NodeID, ParameterList};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDefinition {
    pub documentation: Option<Documentation>,
    pub name: String,
    pub name_location: String,
    pub parameters: ParameterList,
    pub src: String,
    pub id: NodeID,
}

impl Display for ErrorDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("error {}{}", self.name, self.parameters))
    }
}
