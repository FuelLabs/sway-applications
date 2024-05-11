<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/english-auction_dark.png">
        <img alt="light theme" src=".docs/english-auction_light.png">
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

An English Auction is where a seller auctions off an asset with an initial price and a reserve price. Users will then begin bidding on the asset until the bidding period has ended or the reserve has been met. Upon completion, users will withdraw either their original deposits or their newly purchased assets depending on the outcome.

The English Auction application implements this idea in a decentralized manner without the need for a 3rd party and with strong settlement assurances. The application has been designed to utilize native assets and NFTs enabling users to auction off native assets / NFTs and place bids using native assets / NFTs.

More information can be found in the [specification](./SPECIFICATION.md).

## Repository structure

The project consists of a smart contract.

```sh
english-auction
├── auction-contract
│   ├── src/main.sw
│   └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User interface

TODO: The user interface does not currently exist.

### Project

In order to run the subsequent commands change into the following directory `/path/to/english-auction/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
