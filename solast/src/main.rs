mod analysis;
mod brownie;
mod hardhat;
mod truffle;
mod todo_list;

use solidity::ast::*;
use std::{collections::HashSet, env, fs::File, io, path::PathBuf};

type VisitorConstructor = fn() -> Box<dyn AstVisitor>;
type VisitorEntry = (&'static str, VisitorConstructor);

const VISITOR_TYPES: &[VisitorEntry] = &[
    ("no_spdx_identifier", || Box::new(analysis::NoSpdxIdentifierVisitor)),
    ("floating_solidity_version", || Box::new(analysis::FloatingSolidityVersionVisitor)),
    ("node_modules_imports", || Box::new(analysis::NodeModulesImportsVisitor)),
    ("redundant_imports", || Box::new(analysis::RedundantImportsVisitor::default())),
    ("abstract_contracts", || Box::new(analysis::AbstractContractsVisitor)),
    ("large_literals", || Box::new(analysis::LargeLiteralsVisitor)),
    ("tight_variable_packing", || Box::new(analysis::TightVariablePackingVisitor::default())),
    ("redundant_getter_function", || Box::new(analysis::RedundantGetterFunctionVisitor)),
    ("require_without_message", || Box::new(analysis::RequireWithoutMessageVisitor)),
    ("state_variable_shadowing", || Box::new(analysis::StateVariableShadowingVisitor)),
    ("explicit_variable_return", || Box::new(analysis::ExplicitVariableReturnVisitor::default())),
    ("unused_return", || Box::new(analysis::UnusedReturnVisitor)),
    ("storage_array_loop", || Box::new(analysis::StorageArrayLoopVisitor::default())),
    ("external_calls_in_loop", || Box::new(analysis::ExternalCallsInLoopVisitor::default())),
    ("check_effects_interactions", || Box::new(analysis::CheckEffectsInteractionsVisitor::default())),
    ("secure_ether_transfer", || Box::new(analysis::SecureEtherTransferVisitor)),
    ("safe_erc20_functions", || Box::new(analysis::SafeERC20FunctionsVisitor)),
    ("unchecked_erc20_transfer", || Box::new(analysis::UncheckedERC20TransferVisitor::default())),
    ("unpaid_payable_functions", || Box::new(analysis::UnpaidPayableFunctionsVisitor)),
    ("divide_before_multiply", || Box::new(analysis::DivideBeforeMultiplyVisitor)),
    ("comparison_utilization", || Box::new(analysis::ComparisonUtilizationVisitor)),
    ("assignment_comparisons", || Box::new(analysis::AssignmentComparisonsVisitor)),
    ("state_variable_mutability", || Box::new(analysis::StateVariableMutabilityVisitor::default())),
    ("unused_state_variables", || Box::new(analysis::UnusedStateVariablesVisitor::default())),
    ("ineffectual_statements", || Box::new(analysis::IneffectualStatementsVisitor)),
    ("inline_assembly", || Box::new(analysis::InlineAssemblyVisitor)),
    ("unchecked_casting", || Box::new(analysis::UncheckedCastingVisitor)),
    ("unnecessary_pragmas", || Box::new(analysis::UnnecessaryPragmasVisitor)),
    ("missing_return", || Box::new(analysis::MissingReturnVisitor::default())),
    ("redundant_state_variable_access", || Box::new(analysis::RedundantStateVariableAccessVisitor)),
    ("redundant_comparisons", || Box::new(analysis::RedundantComparisonsVisitor)),
    ("assert_usage", || Box::new(analysis::AssertUsageVisitor::default())),
    ("selfdestruct_usage", || Box::new(analysis::SelfdestructUsageVisitor)),
    ("unrestricted_setter_functions", || Box::new(analysis::UnrestrictedSetterFunctionsVisitor)),
    ("manipulatable_balance_usage", || Box::new(analysis::ManipulatableBalanceUsageVisitor)),
    ("redundant_assignments", || Box::new(analysis::RedundantAssignmentsVisitor)),
    ("invalid_using_for_directives", || Box::new(analysis::InvalidUsingForDirectivesVisitor)),
    ("abi_encoding", || Box::new(analysis::AbiEncodingVisitor::default())),
    ("address_balance", || Box::new(analysis::AddressBalanceVisitor)),
    ("address_zero", || Box::new(analysis::AddressZeroVisitor)),
    ("array_assignment", || Box::new(analysis::ArrayAssignmentVisitor)),
];

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next().ok_or_else(|| io::Error::from(io::ErrorKind::BrokenPipe))?;

    let mut path: Option<PathBuf> = None;
    let mut should_print_todo_list = false;
    let mut visitor_names: HashSet<String> = HashSet::new();
    let mut contract_name: Option<String> = None;

    for arg in args {
        match arg {
            arg if arg.starts_with("--") => match &arg.as_str()[2..] {
                "todo-list" | "todo_list" => {
                    should_print_todo_list = true;
                }

                s if s.starts_with("contract=") => {
                    if contract_name.is_some() {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Multiple contracts specified: {} {}", path.unwrap().to_string_lossy(), arg)));
                    }
                    
                    contract_name = Some(s.trim_start_matches("contract=").into());
                }

                s if VISITOR_TYPES.iter().any(|visitor| visitor.0 == s) => {
                    if !visitor_names.contains(s) {
                        visitor_names.insert(s.into());
                    }
                }

                _ => {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid argument: {}", arg)));
                }
            }

            _ => {
                if path.is_some() {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Multiple paths specified: {} {}", path.unwrap().to_string_lossy(), arg)));
                }

                path = Some(PathBuf::from(arg));
            }
        }
    }

    let path = match path {
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path not supplied"))
    };

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, path.to_string_lossy()))
    }
    
    let mut source_units: Vec<SourceUnit> = vec![];

    let brownie_config_path = path.join("brownie-config.yaml");
    let hardhat_config_js_path = path.join("hardhat.config.js");
    let hardhat_config_ts_path = path.join("hardhat.config.ts");
    let truffle_config_path = path.join("truffle-config.js");

    if brownie_config_path.is_file() {
        //
        // TODO: load the brownie config and get the actual build paths
        //

        let build_paths = &[
            path.join("build").join("contracts"),
            path.join("build").join("interfaces"),
        ];

        for build_path in build_paths {
            if !build_path.exists() || !build_path.is_dir() {
                todo!("brownie project not compiled")
            }

            for path in std::fs::read_dir(build_path)? {
                let path = path?.path();

                if !path.is_file() || !path.extension().map(|extension| extension == "json").unwrap_or(false) {
                    continue;
                }

                let file: brownie::File = simd_json::from_reader(File::open(path)?)?;

                if let Some(mut source_unit) = file.ast {
                    if let Some(contract_name) = contract_name.as_deref() {
                        if !source_unit.contract_definitions().iter().any(|c| c.name == contract_name) {
                            continue;
                        }
                    }

                    if !source_units.iter().any(|existing_source_unit| existing_source_unit.absolute_path == source_unit.absolute_path) {
                        source_unit.source = file.source.clone();
                        source_units.push(source_unit);
                    }
                }
            }
        }
    } else if hardhat_config_js_path.is_file() || hardhat_config_ts_path.is_file() {
        let build_path = path.join("artifacts").join("build-info");

        if !build_path.exists() || !build_path.is_dir() {
            todo!("hardhat project not compiled")
        }

        let console_path = PathBuf::new()
            .join("hardhat")
            .join("console.sol")
            .to_string_lossy()
            .to_string();

        for path in std::fs::read_dir(build_path)? {
            let path = path?.path();

            if !path.is_file() || !path.extension().map(|extension| extension == "json").unwrap_or(false) {
                continue;
            }

            let file: hardhat::File = simd_json::from_reader(File::open(path)?)?;

            for (source_path, source) in file.output.sources {
                let mut source_unit = source.ast;

                if source_unit.absolute_path.as_deref().unwrap_or("").ends_with(console_path.as_str()) {
                    continue;
                }
                
                if let Some(contract_name) = contract_name.as_deref() {
                    if !source_unit.contract_definitions().iter().any(|c| c.name == contract_name) {
                        continue;
                    }
                }

                if !source_units.iter().any(|existing_source_unit| existing_source_unit.absolute_path == source_unit.absolute_path) {
                    if let Some(source) = file.input.sources.get(&source_path) {
                        source_unit.source = Some(source.content.clone());
                        source_units.push(source_unit);
                    }
                }
            }
        }
    } else if truffle_config_path.is_file() {
        let build_path = path.join("build").join("contracts");

        if !build_path.exists() || !build_path.is_dir() {
            todo!("truffle project not compiled")
        }

        let migrations_path = PathBuf::new()
            .join("contracts")
            .join("Migrations.sol")
            .to_string_lossy()
            .to_string();

        for path in std::fs::read_dir(build_path)? {
            let path = path?.path();

            if !path.is_file() || !path.extension().map(|extension| extension == "json").unwrap_or(false) {
                continue;
            }

            let file: truffle::File = simd_json::from_reader(File::open(path)?)?;

            if let Some(mut source_unit) = file.ast {
                if source_unit.absolute_path.as_deref().unwrap_or("").ends_with(migrations_path.as_str()) {
                    continue;
                }
                
                if let Some(contract_name) = contract_name.as_deref() {
                    if !source_unit.contract_definitions().iter().any(|c| c.name == contract_name) {
                        continue;
                    }
                }

                if !source_units.iter().any(|existing_source_unit| existing_source_unit.absolute_path == source_unit.absolute_path) {
                    source_unit.source = file.source.clone();
                    source_units.push(source_unit);
                }
            }
        }
    } else {
        unimplemented!("no supported project configuration found")
    }

    source_units.sort_by(|lhs, rhs| {
        let lhs = lhs.absolute_path.as_deref().unwrap_or("");
        let rhs = rhs.absolute_path.as_deref().unwrap_or("");
        lhs.cmp(rhs)
    });

    if should_print_todo_list {
        todo_list::print(source_units.as_slice());
    }

    let mut visitors: Vec<Box<dyn AstVisitor>> = vec![
        Box::new(analysis::SourceUnitVisitor::default()),
    ];

    for &(visitor_name, create_visitor) in VISITOR_TYPES {
        if visitor_names.is_empty() || visitor_names.contains(visitor_name) {
            visitors.push(create_visitor());
        }
    }

    let mut data = AstVisitorData {
        analyzed_paths: HashSet::new(),
        visitors
    };

    for source_unit in source_units.iter() {
        //
        // Skip node_modules imports
        //

        if source_unit.absolute_path.as_deref().unwrap_or("").starts_with('@') {
            continue;
        }

        //
        // Don't analyze the same source unit multiple times
        //

        if let Some(path) = source_unit.absolute_path.as_ref() {
            if data.analyzed_paths.contains(path) {
                continue;
            }

            data.analyzed_paths.insert(path.clone());
        }

        //
        // Visit the source unit
        //

        let mut context = SourceUnitContext {
            source_units: source_units.as_slice(),
            current_source_unit: source_unit
        };

        data.visit_source_unit(&mut context)?;
        data.leave_source_unit(&mut context)?;
    }

    Ok(())
}
