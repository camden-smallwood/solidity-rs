use std::{env, fs::File, io};

pub mod analysis;
pub mod truffle;

fn main() -> io::Result<()> {
    let mut args = env::args();
    let _ = args.next().unwrap();

    let path = match args.next() {
        Some(arg) => arg,
        None => return Ok(())
    };

    let mut files: Vec<truffle::File> = vec![];

    for path in std::fs::read_dir(path)? {
        let path = path?.path();

        match simd_json::from_reader(File::open(path.clone())?) {
            Ok(file) => files.push(file),
            Err(err) => {
                println!("Failed to load file: {}", err);
                
                let value: serde_json::Value = simd_json::from_reader(File::open(path.clone())?)?;
                let object = value.as_object().unwrap();

                if !object.contains_key("ast") {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "No AST defined in scheme file",
                    ));
                }

                let ast = object.get("ast").unwrap().as_object().unwrap();

                if !ast.contains_key("nodes") {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "No nodes defined in AST",
                    ));
                }

                let nodes = ast.get("nodes").unwrap().as_array().unwrap();

                for node in nodes.iter() {
                    if let Err(err) = serde_json::from_value::<solidity::ast::SourceUnitNode>(node.clone()) {
                        let mut object = node.as_object().unwrap().clone();

                        if let Some(node_type) = object.get("nodeType") {
                            match serde_json::from_value::<solidity::ast::NodeType>(node_type.clone()) {
                                Ok(node_type) => {
                                    println!("Failed to parse node of type {:?}", node_type);

                                    if let solidity::ast::NodeType::ContractDefinition = node_type {
                                        if object.contains_key("nodes") {
                                            let nodes = object.get("nodes").unwrap().as_array().unwrap();
                                            let mut failed = false;
                            
                                            for node in nodes.iter() {
                                                if let Err(err) = serde_json::from_value::<solidity::ast::ContractDefinitionNode>(node.clone()) {
                                                    println!("{}: {:#?}", err, node);
                                                    failed = true;
                                                }
                                            }

                                            if !failed {
                                                object.remove("nodes");
                                                println!("{:#?}", object);
                                            }
                                        }
                                    }
                                }

                                Err(_) => {
                                    println!("{}: {:#?}", err, node);
                                }
                            } 
                        }
                    }
                }
            }
        }
    }

    // for file in files.iter() {
    //     let contract_definitions = file.contract_definitions();

    //     if contract_definitions.is_empty() {
    //         continue;
    //     }

    //     println!("{}", file.source_path.as_ref().unwrap());

    //     for contract_definition in contract_definitions {
    //         println!("{}", contract_definition);
    //     }

    //     println!();
    // }

    let call_graph = analysis::CallGraph::build(files.as_slice())?;

    let mut walker = analysis::AstWalker::default();

    walker.visitors.push(Box::new(analysis::SourceUnitVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::NoSpdxIdentifierVisitor));
    walker.visitors.push(Box::new(analysis::FloatingSolidityVersionVisitor));
    walker.visitors.push(Box::new(analysis::NodeModulesImportsVisitor));
    walker.visitors.push(Box::new(analysis::AbstractContractsVisitor));
    walker.visitors.push(Box::new(analysis::LargeLiteralsVisitor::new()));
    walker.visitors.push(Box::new(analysis::RedundantGetterFunctionVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::RequireWithoutMessageVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::ZeroAddressParametersVisitor::new(files.as_slice(), &call_graph)));
    walker.visitors.push(Box::new(analysis::StateVariableShadowingVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::ExplicitVariableReturnVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::UnusedReturnVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::StorageArrayLoopVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::ExternalCallsInLoopVisitor::new(files.as_slice(), &call_graph)));
    walker.visitors.push(Box::new(analysis::ContractLockingEtherVisitor::new(files.as_slice(), &call_graph)));
    walker.visitors.push(Box::new(analysis::CheckEffectsInteractionsVisitor::new(files.as_slice(), &call_graph)));
    walker.visitors.push(Box::new(analysis::RawAddressTransferVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::SafeERC20FunctionsVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::UncheckedERC20TransferVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::UnpaidPayableFunctionsVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::DivideBeforeMultiplyVisitor::new(files.as_slice())));
    walker.visitors.push(Box::new(analysis::ComparisonUtilizationVisitor));

    walker.analyze(files.as_slice())
}
