use serde::{Deserialize, Serialize};
use solidity::ast::SourceUnit;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub contract_name: Option<String>,
    // TODO: abi
    pub metadata: Option<String>,
    pub bytecode: Option<String>,
    pub deployed_bytecode: Option<String>,
    pub source_map: Option<String>,
    pub deployed_source_map: Option<String>,
    pub source: Option<String>,
    pub source_path: Option<String>,
    pub ast: Option<SourceUnit>,
    // TODO: compiler
    // TODO: networks
    // TODO: schemaVersion
    // TODO: updatedAt
    // TODO: devdoc
    // TODO: userdoc
}
