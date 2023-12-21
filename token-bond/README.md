# Tokenized Bond Smart Contract

This is a implementation of a Tokenized Bond contract. It implements
the [CW20 spec with fixed yield and maturity date](../../packages/cw20/README.md) and is designed to
be deployed as is, or imported into other contracts to easily build
cw20-compatible tokens with fixed yield and maturity date .

# Features
-Tokenized representation of a bond.
-Maturity date tracking.
-Yield calculation based on the bond's principal amount.
-Minting and transferring bond tokens.
-Support for standard CW20 functionalities.

## Running this contract
Getting Started
To build and deploy the tokenized bond smart contract, follow these steps:

-Set up the CosmWasm development environment.
-Clone the repository and navigate to the project directory.
-Build the contract using the CosmWasm compiler.
-Deploy the compiled contract to your chosen blockchain network.
-Interact with the contract using compatible wallet applications or custom transactions.

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via: 

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s'  cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw20_base.wasm .
ls -l cw20_base.wasm
sha256sum cw20_base.wasm
```

Or for a production-ready (optimized) build, run a build command in the
the repository root: https://bitbucket.org/codebrew_web/blockhouse_contracts.