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

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
DAO
├── project
│   ├── contracts
│   │   └── DAO-contract
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

In order to run the subsequent commands change into the following directory `/path/to/DAO/project/<here>`.

#### Program compilation

```bash
forc build
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test
```
