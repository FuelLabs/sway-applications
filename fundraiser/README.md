<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/fundraiser-logo-dark-theme.png">
        <img alt="SwayApps Fundraiser Logo" width="400px" src=".docs/fundraiser-logo-light-theme.png">
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

A fundraiser, or crowdfund, is an application where any number of users are able to pledge towards a goal specified by the creator of the campaign. If the target amount is reached, or surpassed, then after the deadline of the campaign the creator is able to take those funds and spend it towards the proposed goal. On the other hand, if the target is not reached then all the users that have pledged are able to withdraw their pledge.

In this case the pledged asset is a native asset on the Fuel network. More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```
fundraiser/
├── project/
|   └── fundraiser-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/fundraiser/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the fundraiser.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
