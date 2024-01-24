<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/escrow-logo-dark-theme.png">
        <img alt="escrow logo" width="400px" src=".docs/escrow-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.49.1" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.49.1-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.22.0" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.22.0-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.53.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.53.0-blue" />
    </a>
</p>

## Overview

An escrow is a neutral third party that holds an asset on behalf of two parties until a transaction has occurred. Once the transaction has taken place the escrow can be resolved and the assets will be transferred as dictated by the contract. This escrow application handles the transaction between an on-chain and off-chain asset.

The off-chain asset can be anything the user desires. For example, some currency which will be used as a payment for a product or perhaps a deed to an estate. Within the blockchain world it all comes down to 1's and 0's, however, those assets can be tokenized and logic can be coded into a contract to handle different cases. But in the end, it still comes down to good faith outside of the blockchain to carry out the obligation.

For this application, the on-chain asset can be any native asset on the Fuel Network and the transaction is considered to be between a buyer and a seller. At the moment, and for the foreseeable future, there is another party (the arbiter) in case there is a dispute that the buyer and seller cannot resolve amongst themselves. More information can be found in the [specification](./project/SPECIFICATION.md).

## Project Structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
escrow
├── project
│   ├── contracts
│   │   └── escrow-contract
│   │       ├── src/main.sw
│   │       └── tests/harness.rs
│   ├── README.md
│   └── SPECIFICATION.md
├── ui
│   ├── README.md
│   └── SPECIFICATION.md
└── README.md
```

## Running the project

### User interface

TODO: The user interface does not currently exist therefore its [README.md](ui/README.md) and [SPECIFICATION.md](ui/SPECIFICATION.md) are empty.

### Project

In order to run the subsequent commands change into the following directory `/path/to/escrow/project/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
