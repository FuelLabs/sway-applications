<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/template-logo-dark-theme.png">
        <img alt="SwayApps Template Logo" width="400px" src=".docs/template-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.37.0" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.37.0-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.17.9" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.17.9-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.40.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.40.0-blue" />
    </a>
</p>

## Overview

TODO: A summary of the application should be written here.

More information about the contract can be found [here](./SPECIFICATION.md).

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
Template
├── template-contract
│   ├── src/main.sw
│   └── tests/harness.rs
├── libraries
├── predicates
├── scripts
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User interface

TODO: The user interface does not currently exist.

### Project

In order to run the subsequent commands change into the following directory `/path/to/Template/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```