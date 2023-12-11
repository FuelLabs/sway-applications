<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/fundraiser-logo-dark-theme.png">
        <img alt="SwayApps Fundraiser Logo" width="400px" src=".docs/fundraiser-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.48.1" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.48.1-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.21.0" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.21.0-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.53.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.53.0-blue" />
    </a>
</p>

## Overview

A fundraiser, or crowdfund, is an application where any number of users are able to pledge towards a goal specified by the creator of the campaign. If the target amount is reached, or surpassed, then after the deadline of the campaign the creator is able to take those funds and spend it towards the proposed goal. On the other hand, if the target is not reached then all the users that have pledged are able to withdraw their pledge.

In this case the pledged asset is a native asset on the Fuel network. More information can be found in the [specification](./project/SPECIFICATION.md).

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
fundraiser
├── project
│   ├── contracts
│   │   └── fundraiser-contract
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

In order to run the subsequent commands change into the following directory `/path/to/fundraiser/project/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
