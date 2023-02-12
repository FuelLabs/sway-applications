<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/fractional-NFT-dark.png">
        <img alt="light theme" src=".docs/fractional-NFT-light.png">
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

The Fractional NFT Application will lock an NFT into a fractional-NFT(f-NFT) contract and allow users to purchase the newly minted fractionalized tokens. These tokens can then be bought and sold on an AMM or if a buyback is initiated, return them to the distribution contract. If all tokens are returned, the admin may unlock the NFT from the f-NFT contract and regain full ownership.

More information can be found in the [specification](./SPECIFICATION.md).

## Repository Structure

The project consists of a smart contract.

```
fractional-NFT/
├── project/
|   └── fractional-NFT-contract/
|   |   ├── src/main.sw
|   |   └── tests/harness.rs
|   └── token-distributor-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/fractional-NFT/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the fractional NFT.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
