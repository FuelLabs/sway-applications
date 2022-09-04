<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/airdrop-distributor_dark.png">
        <img alt="airdrop-distributor logo" width="400px" src=".docs/airdrop-distributor_light.png">
    </picture>
</p>

## Overview

An airdrop is an application where a set number of users are able to claim a specific number of tokens. In today's ecosystem, this is often used to distribute tokens to an application's user base that has previously interacted with their project. 

In order to verifiably prove that a user has a claim to an airdrop and avoiding the expensive transaction of storing every address on chain, a Merkle Proof is used. By storing the Merkle root, a single `b256` hash, the airdrop-distributor application can cryptographically prove a user's validity to their claim.

> **Note** This application implements the [Binary Merkle Proof Verification Library](https://github.com/FuelLabs/sway-libs/tree/master/sway_libs/src/merkle_proof).

More information can be found in the [specification](./SPECIFICATION.md).

### Current state of the application

- The smart contracts are deemed to be feature complete for now but changes will be made as updates to the Sway Language are released
- The user interface does not currently exist

## Project Structure

The project consists of 2 smart contracts and a user interface which the user can interact with.

<!--Only show most important files e.g. script to run, build etc.-->

```
airdrop/
├── airdrop-distributor/
|    └── src/main.sw
|    └── tests/harness.rs
├── simple-token/
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

In order to run the tests you will need to build both the simple-token and airdrop-distributor projects. The airdrop-distributor is dependent on the simple-token project, as a result simple-token MUST be compiled first.

First, make sure that you are in the root of the simple-token project i.e. `/path/to/airdrop/simple-token/<you are here>`.

There is one command required to run the tests for simple-token. This will both build and test the simple-token project.

1. Run the tests

   ```bash
   forc test
   ```

Second, make sure that you are in the root of the airdrop-distributor project i.e. `/path/to/airdrop/airdrop-distributor/<you are here>`

As the simple-token project has been compiled with the previous command, there is one command required to run the tests for airdrop-distributor. This will both build and test the airdrop-distributor project.

1. Run the tests

   ```bash
   forc test
   ```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the fundraiser.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!

## Contributing

Check [CONTRIBUTING.md](../CONTRIBUTING.md) for more info!
