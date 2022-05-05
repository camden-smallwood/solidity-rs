use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use solidity::ast::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputSource {
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub language: String,
    pub sources: HashMap<String, InputSource>,
    pub settings: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputSource {
    pub ast: SourceUnit,
    pub id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub contracts: HashMap<String, serde_json::Value>,
    pub sources: HashMap<String, OutputSource>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: String,
    #[serde(rename = "_format")]
    pub format: String,
    pub solc_version: String,
    pub solc_long_version: String,
    pub input: Input,
    pub output: Output,
}
