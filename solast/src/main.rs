use std::{env, fs::File, io, path::PathBuf};

pub mod analysis;
pub mod truffle;

fn main() -> io::Result<()> {
    let mut args = env::args();
    let _ = args.next().unwrap();

    let path = match args.next() {
        Some(arg) => PathBuf::from(arg),
        None => return Ok(())
    };

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, path.to_string_lossy()))
    }
    
    let mut source_units: Vec<solidity::ast::SourceUnit> = vec![];

    let truffle_config_path = path.join("truffle-config.js");

    if truffle_config_path.is_file() && truffle_config_path.exists() {
        let build_path = path.join("build").join("contracts");

        if !build_path.exists() || !build_path.is_dir() {
            todo!("truffle project not compiled")
        }

        for path in std::fs::read_dir(build_path)? {
            let path = path?.path();

            if !path.exists() || !path.is_file() || !path.extension().map(|extension| extension == "json").unwrap_or(false) {
                continue;
            }
            
            let file: truffle::File = simd_json::from_reader(File::open(path)?)?;

            if let Some(source_unit) = file.ast {
                source_units.push(source_unit);
            }
        }
    } else {
        todo!("truffle config not found; implement support for other project types")
    }

    let call_graph = analysis::CallGraph::build(source_units.as_slice())?;

    let mut walker = analysis::AstWalker {
        visitors: vec![
            Box::new(analysis::SourceUnitVisitor::new(source_units.as_slice())),
            Box::new(analysis::NoSpdxIdentifierVisitor),
            Box::new(analysis::FloatingSolidityVersionVisitor),
            Box::new(analysis::NodeModulesImportsVisitor),
            Box::new(analysis::AbstractContractsVisitor),
            Box::new(analysis::LargeLiteralsVisitor::default()),
            Box::new(analysis::RedundantGetterFunctionVisitor::new(source_units.as_slice())),
            Box::new(analysis::RequireWithoutMessageVisitor::new(source_units.as_slice())),
            Box::new(analysis::StateVariableShadowingVisitor::new(source_units.as_slice())),
            Box::new(analysis::ExplicitVariableReturnVisitor::default()),
            Box::new(analysis::UnusedReturnVisitor::new(source_units.as_slice())),
            Box::new(analysis::StorageArrayLoopVisitor::new(source_units.as_slice())),
            Box::new(analysis::ExternalCallsInLoopVisitor::new(source_units.as_slice(), &call_graph)),
            Box::new(analysis::CheckEffectsInteractionsVisitor::new(source_units.as_slice(), &call_graph)),
            Box::new(analysis::RawAddressTransferVisitor::new(source_units.as_slice())),
            Box::new(analysis::SafeERC20FunctionsVisitor::new(source_units.as_slice())),
            Box::new(analysis::UncheckedERC20TransferVisitor::new(source_units.as_slice())),
            Box::new(analysis::UnpaidPayableFunctionsVisitor::new(source_units.as_slice())),
            Box::new(analysis::DivideBeforeMultiplyVisitor::new(source_units.as_slice())),
            Box::new(analysis::ComparisonUtilizationVisitor),
            Box::new(analysis::AssignmentComparisonsVisitor),
        ],
        ..Default::default()
    };

    walker.analyze(source_units.as_slice())
}
