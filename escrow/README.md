<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/escrow-logo-dark-theme.png">
        <img alt="escrow logo" width="400px" src=".docs/escrow-logo-light-theme.png">
    </picture>
</p>

## Overview

An escrow is a neutral third party that holds an asset on behald of two parties until a transaction has occured. Once the transaction has taken place the escrow can be resolved and the assets will be transferred as dictated by the contract. 

Outside of the blockchain the asset can be pretty much anything such as currency which will be used as a payment for a product or perhaps a deed to an estate. In the blockchain world it all comes down to 1s and 0s however those assets can be tokenized and logic can be coded into a contract to handle those cases but it still comes down to good faith outside of the blockchain to carry out the obligation.

In this particular case the asset can be any native asset on the Fuel network and the transaction is considered to be between a buyer and a seller. At the moment, and for the foreseeable future, there is another party (the arbiter) in case there is a dispute that the buyer and seller cannot resolve amongst themselves. More information can be found in the [specification](./SPECIFICATION.md).

### Current state of the application

- The smart contract is deemed to be feature complete for now
- The user interface does not currently exist but is a work in progress therefore there is nothing for the user to interact with

## Project Structure

The project consists of a smart contract and a user interface which the user can interact with.

<!--Only show most important files e.g. script to run, build etc.-->

```
escrow/
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

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/escrow/<you are here>`

There are two commands required to run the tests

1. Build the asset used for depositing into the escrow
   
   ```bash
   forc build --path tests/artifacts/asset/
   ```

2. Run the tests

   ```bash
   forc test
   ```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the escrow.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!

## Contributing

Check [CONTRIBUTING.md](../CONTRIBUTING.md) for more info!
