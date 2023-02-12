<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/timelock-logo-dark-theme.png">
        <img alt="SwayApps Timelock Logo" width="400px" src=".docs/timelock-logo-light-theme.png">
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

The purpose of a timelock is to restrict the execution of a transaction to some window of time. The transaction usually involves a transfer of funds e.g. via an escrow, vesting schedule, deferred payment etc. however, it may also be used for valueless execution i.e. calls to a contract to perform computation.

The transaction arguments are hashed and stored in a queue awaiting a subsequent call for execution. A user may choose to execute the transaction during the window of time or cancel the transaction by removing it from the queue.

More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```
timelock/
├── project/
|   └── timelock-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/timelock/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the timelock.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
