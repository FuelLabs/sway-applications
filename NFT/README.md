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

## Overview

A non-fungible token (NFT) is a unique token that has an identifier which distinguishes itself from other tokens within the same token contract. At its core, there is nothing inherently special or unique with this implementation of an NFT besides the token ID. While it is commonly associated with artwork / collectibles, there are many greater utilities beyond that which have yet to be written for the Fuel Network.

> **Note** This application implements the [NFT Library](https://github.com/FuelLabs/sway-libs/tree/master/libs/nft/src) and inherits its specification.

More information can be found in the NFT Library [specification](https://github.com/FuelLabs/sway-libs/blob/master/libs/nft/SPECIFICATION.md).

## Project Structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```
NFT/
├── project/
|   └── NFT-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
└── README.md
```

## Running the project

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/NFT/<you are here>`

Build the contract:

```bash
forc build
```

Run the tests:

```bash
cargo test
```
