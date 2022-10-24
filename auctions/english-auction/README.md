<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/english-auction_dark.png">
        <img alt="light theme" src=".docs/english-auction_light.png">
    </picture>
</p>

## Overview

An english auction is an auction is where a seller places an asset on auction with a initial price and a reserve price. Bidders then begin bidding until the bidding period has ended or the reserve has been met. The English Auction application implements this idea in a decentralized manner without the need for a 3rd party and with strong settlement assurances. 

The application has been designed to utilize native assets and NFTs enabling users to auction off native assets / NFTs and place bids using native assets / NFTs. 

> **Note** This application currently only supports selling and bidding of a single NFT. Support for this functionality can be tracked [here](https://github.com/FuelLabs/sway/issues/2465).

More information can be found in the [specification](./SPECIFICATION.md).

### Current state of the application

Information on the current state of the application can be found in the [Application Progress](../APPLICATION_PROGRESS.md#decentralized-apps) file.

## Repository Structure

The project consists of a smart contract and a user interface which the user can interact with.

```
english-auction/
├── contract/
|    └── src/main.sw
|    └── tests/harness.rs
├── frontend/
|    └── Directories & files
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/english-auction/<you are here>`

There are three commands required to run the tests

1. Build the native token asset used for selling and bidding in the auction
   
   ```bash
   forc build --path tests/artifacts/asset
   ```

1. Build the NFT asset used for selling and bidding in the auction
   
   ```bash
   forc build --path ../../NFT/
   ```

3. Run the tests

   ```bash
   forc test
   ```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the english-auction.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!