use std::{collections::HashSet, env, fs::File, io, path::PathBuf};

mod analysis;
mod truffle;
mod todo_list;

const VISITORS: &'static [(&'static str, fn() -> Box<dyn analysis::AstVisitor>)] = &[
    ("no_spdx_identifier", || Box::new(analysis::NoSpdxIdentifierVisitor)),
    ("floating_solidity_version", || Box::new(analysis::FloatingSolidityVersionVisitor)),
    ("node_modules_imports", || Box::new(analysis::NodeModulesImportsVisitor)),
    ("abstract_contracts", || Box::new(analysis::AbstractContractsVisitor)),
    ("large_literals", || Box::new(analysis::LargeLiteralsVisitor::default())),
    ("tight_variable_packing", || Box::new(analysis::TightVariablePackingVisitor::default())),
    ("redundant_getter_function", || Box::new(analysis::RedundantGetterFunctionVisitor)),
    ("require_without_message", || Box::new(analysis::RequireWithoutMessageVisitor::default())),
    ("state_variable_shadowing", || Box::new(analysis::StateVariableShadowingVisitor)),
    ("explicit_variable_return", || Box::new(analysis::ExplicitVariableReturnVisitor::default())),
    ("unused_return", || Box::new(analysis::UnusedReturnVisitor)),
    ("storage_array_loop", || Box::new(analysis::StorageArrayLoopVisitor::default())),
    ("external_calls_in_loop", || Box::new(analysis::ExternalCallsInLoopVisitor::default())),
    ("check_effects_interactions", || Box::new(analysis::CheckEffectsInteractionsVisitor::default())),
    ("raw_address_transfer", || Box::new(analysis::RawAddressTransferVisitor::default())),
    ("safe_erc20_functions", || Box::new(analysis::SafeERC20FunctionsVisitor::default())),
    ("unchecked_erc20_transfer", || Box::new(analysis::UncheckedERC20TransferVisitor::default())),
    ("unpaid_payable_functions", || Box::new(analysis::UnpaidPayableFunctionsVisitor)),
    ("divide_before_multiply", || Box::new(analysis::DivideBeforeMultiplyVisitor)),
    ("comparison_utilization", || Box::new(analysis::ComparisonUtilizationVisitor)),
    ("assignment_comparisons", || Box::new(analysis::AssignmentComparisonsVisitor)),
    ("state_variable_mutability", || Box::new(analysis::StateVariableMutabilityVisitor::default())),
    ("unused_state_variables", || Box::new(analysis::UnusedStateVariablesVisitor::default())),
    ("ineffectual_statements", || Box::new(analysis::IneffectualStatementsVisitor)),
    ("inline_assembly", || Box::new(analysis::InlineAssemblyVisitor::default())),
    ("unchecked_casting", || Box::new(analysis::UncheckedCastingVisitor)),
    ("unnecessary_pragmas", || Box::new(analysis::UnnecessaryPragmasVisitor)),
    ("missing_return", || Box::new(analysis::MissingReturnVisitor::default())),
    ("redundant_state_variable_access", || Box::new(analysis::RedundantStateVariableAccessVisitor)),
    ("unnecessary_comparisons", || Box::new(analysis::UnnecessaryComparisonsVisitor)),
    ("assert_usage", || Box::new(analysis::AssertUsageVisitor::default())),
    ("unrestricted_setter_functions", || Box::new(analysis::UnrestrictedSetterFunctionsVisitor)),
    ("manipulatable_balance_usage", || Box::new(analysis::ManipulatableBalanceUsageVisitor)),
    ("redundant_assignments", || Box::new(analysis::RedundantAssignmentsVisitor)),
];

fn main() -> io::Result<()> {
    let mut args = env::args();
    let _ = args.next().unwrap();

    let mut path: Option<PathBuf> = None;
    let mut should_print_todo_list = false;
    let mut visitor_names: HashSet<String> = HashSet::new();
    let mut contract_name: Option<String> = None;

    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None => break
        };

        if arg.starts_with("--") {
            match &arg.as_str()[2..] {
                "todo-list" => {
                    should_print_todo_list = true;
                }

                s if s.starts_with("contract=") => {
                    if contract_name.is_some() {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Multiple contracts specified: {} {}", path.unwrap().to_string_lossy(), arg)));
                    }
                    
                    contract_name = Some(s.trim_start_matches("contract=").into());
                }

                s if VISITORS.iter().find(|visitor| visitor.0 == s).is_some() => {
                    if !visitor_names.contains(s) {
                        visitor_names.insert(s.into());
                    }
                }

                _ => {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid argument: {}", arg)));
                }
            }
        } else {
            if path.is_some() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Multiple paths specified: {} {}", path.unwrap().to_string_lossy(), arg)));
            }

            path = Some(PathBuf::from(arg));
        }
    }

    let path = match path {
        Some(path) => path,
        None => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path not supplied"));
        }
    };

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, path.to_string_lossy()))
    }
    
    let mut source_units: Vec<solidity::ast::SourceUnit> = vec![];

    let truffle_config_path = path.join("truffle-config.js");

    if truffle_config_path.is_file() {
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
                if let Some(contract_name) = contract_name.as_ref().map(String::as_str) {
                    if !source_unit.contract_definitions().iter().find(|c| c.name == contract_name).is_some() {
                        continue;
                    }
                }
                
                source_units.push(source_unit);
            }
        }

        source_units.sort_by(|lhs, rhs| lhs.absolute_path.as_ref().map(String::as_str).unwrap_or("").cmp(rhs.absolute_path.as_ref().map(String::as_str).unwrap_or("")));
    } else {
        todo!("truffle config not found; implement support for other project types")
    }

    if should_print_todo_list {
        todo_list::print(source_units.as_slice());
    }

    let mut visitors: Vec<Box<dyn analysis::AstVisitor>> = vec![
        Box::new(analysis::SourceUnitVisitor::new(source_units.as_slice())),
    ];

    for &(visitor_name, create_visitor) in VISITORS {
        if visitor_names.is_empty() || visitor_names.contains(visitor_name) {
            visitors.push(create_visitor());
        }
    }

    analysis::visit_source_units(visitors, source_units.as_slice())
}
