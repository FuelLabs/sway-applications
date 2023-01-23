<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/english-auction_dark.png">
        <img alt="light theme" src=".docs/english-auction_light.png">
    </picture>
</p>

## Overview

An English Auction is where a seller auctions off an asset with an initial price and a reserve price. Users will then begin bidding on the asset until the bidding period has ended or the reserve has been met. Upon completion, users will withdraw either their original deposits or their newly purchased assets depending on the outcome.

The English Auction application implements this idea in a decentralized manner without the need for a 3rd party and with strong settlement assurances. The application has been designed to utilize native assets and NFTs enabling users to auction off native assets / NFTs and place bids using native assets / NFTs.

More information can be found in the [specification](./SPECIFICATION.md).

## Repository Structure

The project consists of a smart contract.

```
english-auction/
├── docker/
├── packages
|   └── app/
|   └── config/
|   └── contracts/project/auction-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
|   └── scripts/
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

To run the frontend locally first make sure that your are in the root of this project directory i.e `/path/to/english-auction/`

Install dependencies
```bash
pnpm install
```

Run a local node and setup contracts
```bash
pnpm services:setup
```

Run web app
```bash
pnpm dev
```

You can now interact with the web app on `http://localhost:3000`

### User Interface E2E Tests

In order to run the user interface e2e tests make sure that you are in the root of this directory i.e `/path/to/english-auction/`

Run a local node and setup contracts in test env
```bash
pnpm services:setup-test
```

Run test
```bash
pnpm test
```

### Rust Unit Tests

In order to run the rust unit tests make sure that you are in this directory `/packages/contracts/english-auction/project/auction-contract/<you are here>`

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
