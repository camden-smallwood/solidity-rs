use serde::{Deserialize, Serialize};
use solidity::ast::{ContractDefinition, EnumDefinition, FunctionDefinition, NodeID, StructDefinition};

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
    pub ast: Option<solidity::ast::SourceUnit>,
    // TODO: compiler
    // TODO: networks
    // TODO: schemaVersion
    // TODO: updatedAt
    // TODO: devdoc
    // TODO: userdoc
}

impl File {
    pub fn contract_definition(&self, id: NodeID) -> Option<&ContractDefinition> {
        match self.ast.as_ref() {
            Some(ast) => ast.contract_definition(id),
            None => None,
        }
    }

    pub fn contract_definitions(&self) -> Vec<&ContractDefinition> {
        match self.ast.as_ref() {
            Some(ast) => ast.contract_definitions(),
            None => vec![],
        }
    }

    pub fn struct_definition(&self, id: NodeID) -> Option<&StructDefinition> {
        match self.ast.as_ref() {
            Some(ast) => ast.struct_definition(id),
            None => None,
        }
    }

    pub fn enum_definition(&self, id: NodeID) -> Option<&EnumDefinition> {
        match self.ast.as_ref() {
            Some(ast) => ast.enum_definition(id),
            None => None,
        }
    }

    pub fn function_definition(&self, id: NodeID) -> Option<&FunctionDefinition> {
        match self.ast.as_ref() {
            Some(ast) => ast.function_definition(id),
            None => None,
        }
    }

    pub fn function_and_contract_definition(
        &self,
        id: NodeID,
    ) -> Option<(&ContractDefinition, &FunctionDefinition)> {
        match self.ast.as_ref() {
            Some(ast) => ast.function_and_contract_definition(id),
            None => None,
        }
    }
}
