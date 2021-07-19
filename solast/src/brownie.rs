use serde::{Deserialize, Serialize};
use solidity::ast::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    // TODO: abi
    // TODO: all_source_paths
    pub ast: Option<SourceUnit>,
    pub bytecode: Option<String>,
    // TODO: bytecode_sha1
    // TODO: compiler
    pub contract_name: Option<String>,
    // TODO: coverage_map
    // TODO: dependencies
    pub deployed_bytecode: Option<String>,
    pub deployed_source_map: Option<String>,
    // TODO: language
    // TODO: natspec
    // TODO: offset
    // TODO: opcodes
    // TODO: pc_map
    // TODO: sha1
    pub source: Option<String>,
    pub source_map: Option<String>,
    pub source_path: Option<String>,
    // TODO: type
}
