<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/counter-script-logo-dark-theme.png">
        <img alt="SwayApps Counter Script Logo" width="400px" src=".docs/counter-script-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.40.1" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.40.1-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.18.3" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.18.3-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.43.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.43.0-blue" />
    </a>
</p>

## Overview

The script is an example which demonstrates how to call contracts in an arbitrary order and test them using the Rust SDK.

The script manipulates contract state through ABI calls.

More information can be found in the [specification](./project/SPECIFICATION.md).

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
counter_script
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

In order to run the subsequent commands change into the following directory `/path/to/counter_script/project/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
