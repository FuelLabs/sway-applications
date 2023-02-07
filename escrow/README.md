<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/escrow-logo-dark-theme.png">
        <img alt="escrow logo" width="400px" src=".docs/escrow-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.33.1" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.33.1-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.15.3" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.15.3-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.34.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.34.0-blue" />
    </a>
</p>

## Overview

An escrow is a neutral third party that holds an asset on behalf of two parties until a transaction has occurred. Once the transaction has taken place the escrow can be resolved and the assets will be transferred as dictated by the contract. This escrow application handles the transaction between an on-chain and off-chain asset.

The off-chain asset can be anything the user desires. For example, some currency which will be used as a payment for a product or perhaps a deed to an estate. Within the blockchain world it all comes down to 1's and 0's, however, those assets can be tokenized and logic can be coded into a contract to handle different cases. But in the end, it still comes down to good faith outside of the blockchain to carry out the obligation.

For this application, the on-chain asset can be any native asset on the Fuel Network and the transaction is considered to be between a buyer and a seller. At the moment, and for the foreseeable future, there is another party (the arbiter) in case there is a dispute that the buyer and seller cannot resolve amongst themselves. More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```
escrow/
├── project/
|   └── escrow-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/escrow/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the escrow.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
