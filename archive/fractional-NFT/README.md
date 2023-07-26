<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/fractional-NFT-dark.png">
        <img alt="light theme" src=".docs/fractional-NFT-light.png">
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

> **Warning**
> This application have been temporarily archived until the release of the [Sway multi-token standard](https://github.com/FuelLabs/rfcs/pull/17). In its current form, it does not comply with this standard and as such should be considered outdated.
> It is inadvisable to use this application for production purposes.

## Overview

The Fractional NFT Application will lock an NFT into a fractional-NFT(f-NFT) contract and allow users to purchase the newly minted fractionalized tokens. These tokens can then be bought and sold on an AMM or if a buyback is initiated, return them to the distribution contract. If all tokens are returned, the admin may unlock the NFT from the f-NFT contract and regain full ownership.

More information can be found in the [specification](./project/SPECIFICATION.md).

## Repository structure

The project consists of two smart contracts.

```sh
fractional-NFT
├── project
│   ├── contracts
│   │   ├── fractional-NFT-contract
│   │   │   ├── src/main.sw
│   │   │   └── tests/harness.rs
│   │   └── token-distributor-contract
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

In order to run the subsequent commands change into the following directory `/path/to/fractional-NFT/project/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
