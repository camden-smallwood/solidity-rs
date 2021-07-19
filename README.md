# SolAST
Solidity 0.8.X AST parsing and analysis in Rust.

Some legacy versions of Solidity are inherently supported (0.5.X-0.7.X), but the focus is primarily on Solidity 0.8.X and above.

## Analyzers

- [x] `no_spdx_identifier`
- [x] `floating_solidity_version`
- [x] `node_modules_imports`
- [x] `redundant_imports`
- [x] `abstract_contracts`
- [x] `large_literals`
- [ ] ~~`tight_variable_packing`~~ (WIP)
- [x] `redundant_getter_function`
- [x] `require_without_message`
- [x] `state_variable_shadowing`
- [x] `explicit_variable_return`
- [x] `unused_return`
- [x] `storage_array_loop`
- [x] `external_calls_in_loop`
- [x] `check_effects_interactions`
- [x] `raw_address_transfer`
- [x] `safe_erc20_functions`
- [x] `unchecked_erc20_transfer`
- [x] `unpaid_payable_functions`
- [ ] ~~`divide_before_multiply`~~ (WIP)
- [ ] ~~`comparison_utilization`~~ (WIP)
- [x] `assignment_comparisons`
- [x] `state_variable_mutability`
- [x] `unused_state_variables`
- [x] `ineffectual_statements`
- [x] `inline_assembly`
- [ ] ~~`unchecked_casting`~~ (WIP)
- [ ] ~~`unnecessary_pragmas`~~ (WIP)
- [x] `missing_return`
- [ ] ~~`redundant_state_variable_access`~~ (WIP)
- [ ] ~~`redundant_comparisons`~~ (WIP)
- [x] `assert_usage`
- [x] `selfdestruct_usage`
- [ ] ~~`unrestricted_setter_functions`~~ (WIP)
- [ ] ~~`manipulatable_balance_usage`~~ (WIP)
- [ ] ~~`redundant_assignments`~~ (WIP)
- [x] `invalid_using_for_directives`
- [x] `abi_encoding`

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

Use your favorite text editor to change the `solc` version in `truffle-config.js` to `0.8.6` (or the relevant `0.8.X`).

```Json
module.exports = {
  networks: {},
  mocha: {},
  compilers: {
    solc: {
      version: "0.8.6",
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
