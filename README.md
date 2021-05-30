# solast
Solidity 0.8.X AST parsing and analysis in Rust.

Some legacy versions of Solidity are inherently supported, but the focus is 0.8.X and forward.

## Analyzers

- [x] `no-spdx-identifier`
- [x] `floating-solidity-version`
- [x] `node-modules-imports`
- [x] `large-literals`
- [x] `redundant-getter-function`
- [x] `require-without-message`
- [x] `zero-address-parameters`
- [x] `state-variable-shadowing`
- [x] `explicit-variable-return`
- [x] `unused-return`
- [x] `storage-array-loop`
- [x] `external-calls-in-loop`
- [x] `check-effects-interactions`
- [x] `raw-address-transfer`
- [x] `safe-erc20-functions`
- [x] `unchecked-erc20-transfer`
- [ ] ~~`divide-before-multiply`~~ (WIP)
- [ ] ~~`comparison-utilization`~~ (WIP)
- [x] `assignment-comparisons`

## Usage

Currently, SolAST requires utilization of a [truffle](https://www.trufflesuite.com/) project.

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

Use your favorite text editor to change the `solc` version in `truffle-config.js` to `0.8.3` (or the relevant `0.8.X`).

```Json
module.exports = {
  networks: {},
  mocha: {},
  compilers: {
    solc: {
      version: "0.8.3",
    }
  }
};
```

Compile your truffle project with `npm i && rm -rf build && truffle compile`.
You should have a `build/contracts/` folder with `*.json` files inside of it afterwards.

Now you can supply the path to this directory to SolAST with the following:
```Shell
cargo run --release -- /path/to/project/build/contracts
```

If you would like to save text output to an `out.txt` file instead of printing to the terminal, use the following:
```Shell
cargo run --release -- /path/to/project/build/contracts > out.txt
```

On the first run it may take a few minutes to optimize and compile, but subsequent runs will be quite fast in release mode.
