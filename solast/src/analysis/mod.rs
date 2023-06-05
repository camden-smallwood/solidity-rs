mod abi_encoding;
mod abstract_contracts;
mod address_balance;
mod address_zero;
mod array_assignment;
mod assert_usage;
mod assignment_comparisons;
mod check_effects_interactions;
mod comparison_utilization;
mod divide_before_multiply;
mod explicit_variable_return;
mod external_calls_in_loop;
mod floating_solidity_version;
mod ineffectual_statements;
mod inline_assembly;
mod invalid_using_for_directives;
mod large_literals;
mod manipulatable_balance_usage;
mod missing_return;
mod no_spdx_identifier;
mod node_modules_imports;
mod redundant_assignments;
mod redundant_comparisons;
mod redundant_getter_function;
mod redundant_imports;
mod redundant_state_variable_access;
mod require_without_message;
mod safe_erc20_functions;
mod secure_ether_transfer;
mod selfdestruct_usage;
mod state_variable_mutability;
mod state_variable_shadowing;
mod storage_array_loop;
mod tight_variable_packing;
mod unchecked_casting;
mod unchecked_erc20_transfer;
mod unnecessary_pragmas;
mod unpaid_payable_functions;
mod unreferenced_state_variables;
mod unrestricted_setter_functions;
mod unused_return;

use self::{
    abi_encoding::*, abstract_contracts::*, address_balance::*, address_zero::*,
    array_assignment::*, assert_usage::*, assignment_comparisons::*, check_effects_interactions::*,
    comparison_utilization::*, divide_before_multiply::*, explicit_variable_return::*,
    external_calls_in_loop::*, floating_solidity_version::*, ineffectual_statements::*,
    inline_assembly::*, invalid_using_for_directives::*, large_literals::*,
    manipulatable_balance_usage::*, missing_return::*, no_spdx_identifier::*,
    node_modules_imports::*, redundant_assignments::*, redundant_comparisons::*,
    redundant_getter_function::*, redundant_imports::*, redundant_state_variable_access::*,
    require_without_message::*, safe_erc20_functions::*, secure_ether_transfer::*,
    selfdestruct_usage::*, state_variable_mutability::*, state_variable_shadowing::*,
    storage_array_loop::*, tight_variable_packing::*, unchecked_casting::*,
    unchecked_erc20_transfer::*, unnecessary_pragmas::*, unpaid_payable_functions::*,
    unreferenced_state_variables::*, unrestricted_setter_functions::*, unused_return::*,
};

use crate::report::Report;
use solidity::ast::AstVisitor;
use std::{rc::Rc, cell::RefCell};

type VisitorConstructor = fn(report: Rc<RefCell<Report>>) -> Box<dyn AstVisitor>;
type VisitorEntry = (&'static str, VisitorConstructor);

pub const VISITOR_TYPES: &[VisitorEntry] = &[
    ("no_spdx_identifier", |report: Rc<RefCell<Report>>| Box::new(NoSpdxIdentifierVisitor::new(report))),
    ("floating_solidity_version", |report: Rc<RefCell<Report>>| Box::new(FloatingSolidityVersionVisitor::new(report))),
    ("node_modules_imports", |report: Rc<RefCell<Report>>| Box::new(NodeModulesImportsVisitor::new(report))),
    ("redundant_imports", |report: Rc<RefCell<Report>>| Box::new(RedundantImportsVisitor::new(report))),
    ("abstract_contracts", |report: Rc<RefCell<Report>>| Box::new(AbstractContractsVisitor::new(report))),
    ("large_literals", |report: Rc<RefCell<Report>>| Box::new(LargeLiteralsVisitor::new(report))),
    ("tight_variable_packing", |report: Rc<RefCell<Report>>| Box::new(TightVariablePackingVisitor::new(report))),
    ("redundant_getter_function", |report: Rc<RefCell<Report>>| Box::new(RedundantGetterFunctionVisitor::new(report))),
    ("require_without_message", |report: Rc<RefCell<Report>>| Box::new(RequireWithoutMessageVisitor::new(report))),
    ("state_variable_shadowing", |report: Rc<RefCell<Report>>| Box::new(StateVariableShadowingVisitor::new(report))),
    ("explicit_variable_return", |report: Rc<RefCell<Report>>| Box::new(ExplicitVariableReturnVisitor::new(report))),
    ("unused_return", |report: Rc<RefCell<Report>>| Box::new(UnusedReturnVisitor::new(report))),
    ("storage_array_loop", |report: Rc<RefCell<Report>>| Box::new(StorageArrayLoopVisitor::new(report))),
    ("external_calls_in_loop", |report: Rc<RefCell<Report>>| Box::new(ExternalCallsInLoopVisitor::new(report))),
    ("check_effects_interactions", |report: Rc<RefCell<Report>>| Box::new(CheckEffectsInteractionsVisitor::new(report))),
    ("secure_ether_transfer", |report: Rc<RefCell<Report>>| Box::new(SecureEtherTransferVisitor::new(report))),
    ("safe_erc20_functions", |report: Rc<RefCell<Report>>| Box::new(SafeERC20FunctionsVisitor::new(report))),
    ("unchecked_erc20_transfer", |report: Rc<RefCell<Report>>| Box::new(UncheckedERC20TransferVisitor::new(report))),
    ("unpaid_payable_functions", |report: Rc<RefCell<Report>>| Box::new(UnpaidPayableFunctionsVisitor::new(report))),
    ("divide_before_multiply", |report: Rc<RefCell<Report>>| Box::new(DivideBeforeMultiplyVisitor::new(report))),
    ("comparison_utilization", |report: Rc<RefCell<Report>>| Box::new(ComparisonUtilizationVisitor::new(report))),
    ("assignment_comparisons", |report: Rc<RefCell<Report>>| Box::new(AssignmentComparisonsVisitor::new(report))),
    ("state_variable_mutability", |report: Rc<RefCell<Report>>| Box::new(StateVariableMutabilityVisitor::new(report))),
    ("unused_state_variables", |report: Rc<RefCell<Report>>| Box::new(UnusedStateVariablesVisitor::new(report))),
    ("ineffectual_statements", |report: Rc<RefCell<Report>>| Box::new(IneffectualStatementsVisitor::new(report))),
    ("inline_assembly", |report: Rc<RefCell<Report>>| Box::new(InlineAssemblyVisitor::new(report))),
    ("unchecked_casting", |report: Rc<RefCell<Report>>| Box::new(UncheckedCastingVisitor::new(report))),
    ("unnecessary_pragmas", |report: Rc<RefCell<Report>>| Box::new(UnnecessaryPragmasVisitor::new(report))),
    ("missing_return", |report: Rc<RefCell<Report>>| Box::new(MissingReturnVisitor::new(report))),
    ("redundant_state_variable_access", |report: Rc<RefCell<Report>>| Box::new(RedundantStateVariableAccessVisitor::new(report))),
    ("redundant_comparisons", |report: Rc<RefCell<Report>>| Box::new(RedundantComparisonsVisitor::new(report))),
    ("assert_usage", |report: Rc<RefCell<Report>>| Box::new(AssertUsageVisitor::new(report))),
    ("selfdestruct_usage", |report: Rc<RefCell<Report>>| Box::new(SelfdestructUsageVisitor::new(report))),
    ("unrestricted_setter_functions", |report: Rc<RefCell<Report>>| Box::new(UnrestrictedSetterFunctionsVisitor::new(report))),
    ("manipulatable_balance_usage", |report: Rc<RefCell<Report>>| Box::new(ManipulatableBalanceUsageVisitor::new(report))),
    ("redundant_assignments", |report: Rc<RefCell<Report>>| Box::new(RedundantAssignmentsVisitor::new(report))),
    ("invalid_using_for_directives", |report: Rc<RefCell<Report>>| Box::new(InvalidUsingForDirectivesVisitor::new(report))),
    ("abi_encoding", |report: Rc<RefCell<Report>>| Box::new(AbiEncodingVisitor::new(report))),
    ("address_balance", |report: Rc<RefCell<Report>>| Box::new(AddressBalanceVisitor::new(report))),
    ("address_zero", |report: Rc<RefCell<Report>>| Box::new(AddressZeroVisitor::new(report))),
    ("array_assignment", |report: Rc<RefCell<Report>>| Box::new(ArrayAssignmentVisitor::new(report))),
];
