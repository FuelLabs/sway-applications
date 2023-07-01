<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/name-registry-logo-dark-theme.png">
        <img alt="SwayApps Fundraiser Logo" width="400px" src=".docs/name-registry-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.40.1" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.40.1-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.17.9" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.17.9-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.43.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.43.0-blue" />
    </a>
</p>

## Overview

The counter contract interaction script is a simple script example that demonstrates how one can make a script to call contracts in an arbitrary order, also demonstrating how to utilize scripts using the rust sdk, via the test.

A counter contract is deployed, with methods to manipulate the internal counter variable, these methods are called in the script

More information can be found in the [specification](./project/SPECIFICATION.md).

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
name-registry
├── project
│   ├── contract
│   │   └── src/main.sw
│   ├── contract_abi
│   │   └── src/lib.sw
│   ├── script
│   │   └── src/main.sw
│   ├── tests/src/harness.rs
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

In order to run the subsequent commands change into the following directory `/path/to/counter_contract_interaction_script/project/<here>`.

#### Program compilation

```bash
cd contract
forc build --locked
cd ..
cd script
forc build --locked
cd ..
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
