use serde::{Deserialize, Serialize};
use solidity::ast::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub ast: SourceUnit,
}
