<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/timelock-logo-dark-theme.png">
        <img alt="SwayApps Timelock Logo" width="400px" src=".docs/timelock-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.46.0" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.46.0-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.20.5" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.20.5-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.49.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.49.0-blue" />
    </a>
</p>

## Overview

The purpose of a timelock is to restrict the execution of a transaction to some window of time. The transaction usually involves a transfer of funds e.g. via an escrow, vesting schedule, deferred payment etc. however, it may also be used for valueless execution i.e. calls to a contract to perform computation.

The transaction arguments are hashed and stored in a queue awaiting a subsequent call for execution. A user may choose to execute the transaction during the window of time or cancel the transaction by removing it from the queue.

More information can be found in the [specification](./project/SPECIFICATION.md).

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
timelock
├── project
│   ├── contracts
│   │   └── timelock-contract
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

In order to run the subsequent commands change into the following directory `/path/to/timelock/project/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
