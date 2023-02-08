<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/dao-logo-dark-theme.png">
        <img alt="multisig logo" width="400px" src=".docs/dao-logo-light-theme.png">
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

A decentralized autonomous organization (DAO) is akin to an on-chain government where participants are able to cast votes on proposals using some governance token. Various consensus mechanisms may be implemented in order to determine whether a proposal will pass and if that happens then the DAO will begin to operate under the rules of the new proposal. In this implementation the user deposits governance tokens and receives some number of votes that can be cast and recast on different proposals. They can vote in favour or against proposals and they can transform their votes back into the governance tokens if they wish to withdraw.

More information can be found in the [specification](./project/SPECIFICATION.md).

## Project Structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```
DAO/
├── project/
|   └── DAO-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/DAO/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the DAO application.

Check [SPECIFICATION.md](./project/SPECIFICATION.md) for more info!
