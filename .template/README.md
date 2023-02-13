<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/template-logo-dark-theme.png">
        <img alt="SwayApps Template Logo" width="400px" src=".docs/template-logo-light-theme.png">
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

TODO: A summary of the application should be written here.

More information about the contract can be found [here](./project/SPECIFICATION.md) and for the user interface [here](./ui/SPECIFICATION.md).

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
Template
├── project
│   ├── contracts
│   │   └── template-contract
│   │       ├── src/main.sw
│   │       └── tests/harness.rs
│   ├── libraries
│   ├── predicates
│   ├── scripts
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

In order to run the subsequent commands change into the following directory `/path/to/Template/project/<here>`.

#### Program compilation

```bash
forc build
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test
```