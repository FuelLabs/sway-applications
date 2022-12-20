<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/fractional-NFT-dark.png">
        <img alt="light theme" src=".docs/fractional-NFT-light.png">
    </picture>
</p>

## Overview

The Fractional NFT Application will lock an NFT into a fractional-NFT(f-NFT) contract and allow users to purchase the fractionalized tokens. These tokens can then be bought and sold on an AMM or if a buyback is initiated, returned to the distribution contract. If all tokens are returned, the NFT owner may unlock the NFT from the f-NFT contract and regain full ownership.

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
