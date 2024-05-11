<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/airdrop-distributor_dark.png">
        <img alt="airdrop-distributor logo" width="400px" src=".docs/airdrop-distributor_light.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.56.0" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.56.0-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.26.0" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.26.0-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.61.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.61.0-blue" />
    </a>
</p>

## Overview

An airdrop is an application where a set number of users are able to claim a specific amount of an asset. In today's ecosystem, this is often used to distribute assets to an application's user base that has previously interacted with their project.

In order to verifiably prove that a user has a claim to an airdrop and avoiding the expensive transaction of storing every address on chain, a Merkle Proof is used. By storing the Merkle root, a single `b256` hash, the airdrop application can cryptographically prove a user's validity to their claim.

This application implements the [Binary Merkle Proof Verification Library](https://fuellabs.github.io/sway-libs/book/merkle/index.html).

More information can be found in the [specification](SPECIFICATION.md) and [interface](./airdrop-contract/src/interface.sw).

## Project structure

The project consists of two smart contracts.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
airdrop
├── airdrop-contract
│   ├── src/main.sw
│   └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User interface

TODO: The user interface does not currently exist.

### Project

In order to run the subsequent commands change into the following directory `/path/to/airdrop/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
