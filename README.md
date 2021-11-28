# SolAST

Solidity 0.8.X AST parsing and analysis in Rust.

Some legacy versions of Solidity are inherently supported (0.5.X-0.7.X), but the focus is primarily on Solidity 0.8.X and above.

* [Usage](#usage)
* [Analyzers](#analyzers)

## Usage

```
cargo run --release -- [--todo_list] [--contract=<contract_name>] [--analyzer_name1] [--analyzer_nameN] <project_directory>
```

Currently, SolAST requires utilization of either a [truffle](https://www.trufflesuite.com/) project or a [brownie](https://eth-brownie.readthedocs.io/en/stable/) project.

Please file an issue if you would like support for another build system.

If you only have `.sol` files, you can create a quick truffle project by performing the following:

1. Open a terminal.
2. Create a directory for your project to be contained in with `mkdir solidity-project`
3. Move into the newly-created project directory with `cd solidity-project`.
4. Initialize a node module for the project with `npm init -y`.
5. Initialize the truffle project with `truffle init`.
6. Copy all of your `.sol` files into `contracts/`.

```Shell
mkdir solidity-project
cd solidity-project
npm init -y
truffle init
cp ~/Downloads/awesome-contracts/*.sol contracts/
```

Use your favorite text editor to change the `solc` version in `truffle-config.js` to `0.8.10` (or the relevant `0.8.X`).

```Json
module.exports = {
  networks: {},
  mocha: {},
  compilers: {
    solc: {
      version: "0.8.10",
    }
  }
};
```

Compile your truffle project with `npm i && rm -rf build && truffle compile`.
You should have a `build/contracts/` folder with `*.json` files inside of it afterwards.

Now you can supply the path to the truffle project directory to SolAST with the following:
```Shell
cargo run --release -- /path/to/project/
```

If you would like to save text output to an `out.txt` file instead of printing to the terminal, use the following:
```Shell
cargo run --release -- /path/to/project/ > out.txt
```

On the first run it may take a few minutes to optimize and compile, but subsequent runs will be quite fast in release mode.

## Analyzers

*WARNING:* Any analyzer marked (WIP) may not display output or may provide false positives. This is to be expected, as the code has not been fully implemented yet. Please file an issue if you come across a false positive from an analyzer which is not marked (WIP).

| Name | Description |
|-|-|
| `no_spdx_identifier` | Determines if a source file was compiled without a `SPDX` identifier specified. |
| `floating_solidity_version` | Determines if a pragma directive specifies a floating/unlocked Sollidity version. |
| `node_modules_imports` | Determines if an import directive attempts to locally import from the `node_modules` directory. |
| `redundant_imports` | Determines if any import directives are redundant due to the specified path being already previously imported. |
| `abstract_contracts` | Determines if a contract specifies an internal constructor without declaring the contract `abstract`. |
| `large_literals` | Determines if an expression contains a large literal value, which may be difficult to read or interpretted incorrectly. |
| ~~`tight_variable_packing`~~ (WIP) | Determines if a contract or structure contains loose variable packing which can be more efficiently packed in order to decrease the number of required storage slots. |
| `redundant_getter_function` | Determines if a contract contains a function which returns a state variable instead of providing outside access to the state variable. |
| `require_without_message` | Determines if a `require` statement does not contain a message string. |
| `state_variable_shadowing` | Determines if a contract declares a local or state variable which shadows another state variable in the contract's inheritance hierarchy. |
| `explicit_variable_return` | Determines if a function returns local variables explicitly over declaring and utilizing named return variables. |
| `unused_return` | Determines if the values returned from a function call go unused. |
| `storage_array_loop` | Determines if a loop's condition relies on the `length` member of an array state variable. |
| `external_calls_in_loop` | Determines if any functions or modifiers contain any loops which performs calls to external functions. |
| `check_effects_interactions` | Determines if any functions or modifiers ignore the [Check Effects Interactions](https://fravoll.github.io/solidity-patterns/checks_effects_interactions.html) pattern. |
| `secure_ether_transfer` | Determines if any functions or modifiers ignores the [Secure Ether Transfer](https://fravoll.github.io/solidity-patterns/secure_ether_transfer.html) pattern.
| `safe_erc20_functions` | Determines if any functions or modifiers utilize unsafe ERC-20 functionality. |
| `unchecked_erc20_transfer` | Determines if any functions or modifiers perform ERC-20 transfers without checking the value being transferred, which can revert if zero. |
| `unpaid_payable_functions` | Determines if any functions or modifiers perform calls to `payable` functions without paying. |
| ~~`divide_before_multiply`~~ (WIP) | Determines if any functions or modifiers perform multiplication on the result of a division, which can truncate. |
| ~~`comparison_utilization`~~ (WIP) | Determines if an `if` statement's condition contains a comparison without utilizing either compared value in its `true` or `false` branches. |
| `assignment_comparisons` | Determines if any conditional expressions contain assignments, i.e: `require(owner = msg.sender);`, `if (releaseTime = block.timestamp)`, etc. |
| `state_variable_mutability` | Determines if any state variables can be made `constant` or `immutable`. |
| `unused_state_variables` | Determines if any state variables are unused within a contract. |
| `ineffectual_statements` | Determines if any statements are ineffectual, i.e: `balance[msg.sender];` |
| `inline_assembly` | Determines if any functions or modifiers contain inline Yul assembly usage and checks for arbitrary data passing. |
| ~~`unchecked_casting`~~ (WIP) | Determines if a value expression is cast without checking its value beforehand, which can result it invalid values. |
| ~~`unnecessary_pragmas`~~ (WIP) | Determines if any pragma directives are unnecessary for a specific Solidity version. |
| `missing_return` | Determines if a function is missing an explicity return statement without assigning to a named return variable. |
| ~~`redundant_state_variable_access`~~ (WIP) | Determines if any functions or modifiers access state variables multiple times without updating their value between each access. |
| ~~`redundant_comparisons`~~ (WIP) | Determines if any comparisons are redundant, i.e: `true != false`, `uint16(uint8(x)) < 256`, etc. |
| `assert_usage` | Determines if any functions or modifiers utilize `assert(...)`, which should not be used in production. |
| `selfdestruct_usage` | Determines if any functions or modifiers perform a `selfdestruct`. |
| ~~`unrestricted_setter_functions`~~ (WIP) | Determines if any functions allow setting of state variable values without any access restriction or requirements. |
| ~~`manipulatable_balance_usage`~~ (WIP) | Determines if any functions or modifiers contain `balance` usage which can potentially be manipulated, i.e: `address(this).balance`, `IERC20(token).balance()`, etc. |
| ~~`redundant_assignments`~~ (WIP) | Determines if any functions or modifiers perform assignments which are redundant, i.e: `(x, x) = getValues();` |
| `invalid_using_for_directives` | Determines if any using-for directives specify types which do not have functions provided by the specified library. |
| `abi_encoding` | Determines if any functions or modifiers attempt to use `abi.encodePacked` on multiple arguments when any of are variably-sized arrays, which can result in hash collisions. |
