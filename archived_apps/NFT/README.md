<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/nft-logo_white.png">
        <img alt="light theme" src=".docs/nft-logo_black.png">
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

> **Warning**
> This application have been temporarily archived waiting the finalised release of of the [Sway multi-token standard](https://github.com/FuelLabs/rfcs/pull/17) before being updated. In it's current form, it does not comply with this standard and as such should be considered outdated.
> It is not advised to use this applciation for production purposes, use at your own risk

## Overview

A non-fungible token (NFT) is a unique token that has an identifier which distinguishes itself from other tokens within the same token contract. At its core, there is nothing inherently special or unique with this implementation of an NFT besides the token ID. While it is commonly associated with artwork / collectibles, there are many greater utilities beyond that which have yet to be written for the Fuel Network.

> **Note** This application implements the [NFT Library](https://github.com/FuelLabs/sway-libs/tree/master/libs/nft/src) and inherits its specification.

More information can be found in the NFT Library [specification](https://github.com/FuelLabs/sway-libs/blob/master/libs/nft/SPECIFICATION.md).

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
NFT
├── project
│   ├── contracts
│   │   └── NFT-contract
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

In order to run the subsequent commands change into the following directory `/path/to/NFT/project/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
