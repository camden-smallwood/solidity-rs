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
        return Err(
            io::Error::new(
                io::ErrorKind::NotFound,
                path.to_string_lossy()
            )
        )
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

            if path.is_file() && path.extension().map(|extension| extension == "json").unwrap_or(false) {
                let file: truffle::File = simd_json::from_reader(File::open(path)?)?;

                if let Some(source_unit) = file.ast {
                    source_units.push(source_unit);
                }
            }
        }
    } else {
        todo!("truffle config not found; implement support for other project types")
    }

    let call_graph = analysis::CallGraph::build(source_units.as_slice())?;

    let mut walker = analysis::AstWalker::default();

    walker.visitors.push(Box::new(analysis::SourceUnitVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::NoSpdxIdentifierVisitor));
    walker.visitors.push(Box::new(analysis::FloatingSolidityVersionVisitor));
    walker.visitors.push(Box::new(analysis::NodeModulesImportsVisitor));
    walker.visitors.push(Box::new(analysis::AbstractContractsVisitor));
    walker.visitors.push(Box::new(analysis::LargeLiteralsVisitor::new()));
    walker.visitors.push(Box::new(analysis::RedundantGetterFunctionVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::RequireWithoutMessageVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::ZeroAddressParametersVisitor::new(source_units.as_slice(), &call_graph)));
    walker.visitors.push(Box::new(analysis::StateVariableShadowingVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::ExplicitVariableReturnVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::UnusedReturnVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::StorageArrayLoopVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::ExternalCallsInLoopVisitor::new(source_units.as_slice(), &call_graph)));
    walker.visitors.push(Box::new(analysis::ContractLockingEtherVisitor::new(source_units.as_slice(), &call_graph)));
    walker.visitors.push(Box::new(analysis::CheckEffectsInteractionsVisitor::new(source_units.as_slice(), &call_graph)));
    walker.visitors.push(Box::new(analysis::RawAddressTransferVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::SafeERC20FunctionsVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::UncheckedERC20TransferVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::UnpaidPayableFunctionsVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::DivideBeforeMultiplyVisitor::new(source_units.as_slice())));
    walker.visitors.push(Box::new(analysis::ComparisonUtilizationVisitor));
    walker.visitors.push(Box::new(analysis::AssignmentComparisonsVisitor));

    walker.analyze(source_units.as_slice())
}
