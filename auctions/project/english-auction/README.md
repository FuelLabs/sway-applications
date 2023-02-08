<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/english-auction_dark.png">
        <img alt="light theme" src=".docs/english-auction_light.png">
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

An English Auction is where a seller auctions off an asset with an initial price and a reserve price. Users will then begin bidding on the asset until the bidding period has ended or the reserve has been met. Upon completion, users will withdraw either their original deposits or their newly purchased assets depending on the outcome.

The English Auction application implements this idea in a decentralized manner without the need for a 3rd party and with strong settlement assurances. The application has been designed to utilize native assets and NFTs enabling users to auction off native assets / NFTs and place bids using native assets / NFTs. 

More information can be found in the [specification](./SPECIFICATION.md).

## Repository Structure

The project consists of a smart contract.

```
english-auction/
├── project/
|   └── auction-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/auctions/english-auction/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the english-auction.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!