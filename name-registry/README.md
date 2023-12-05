<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/name-registry-logo-dark-theme.png">
        <img alt="SwayApps NameRegistry Logo" width="400px" src=".docs/name-registry-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.47.0" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.47.0-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.20.8" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.20.8-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.53.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.53.0-blue" />
    </a>
</p>

## Overview

The name registry application allows users to use human readable names instead of addresses to send and receive payments, making it easier to transfer cryptocurrency.

A name can be registered for the price of 1 unit of any asset per 100 seconds and it resolves to any Identity the owner sets.
In this implementation the price is paid in the base asset on the Fuel network.

Both the asset and the price per 100 seconds are configuration time constants so can be easily changed to different values.

More information can be found in the [specification](./project/SPECIFICATION.md).

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
name-registry
├── project
│   ├── contracts
│   │   └── registry-contract
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

In order to run the subsequent commands change into the following directory `/path/to/name-registry/project/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
