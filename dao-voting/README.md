<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/dao-logo-dark-theme.png">
        <img alt="multisig logo" width="400px" src=".docs/dao-logo-light-theme.png">
    </picture>
</p>

## Overview

A decentralized autonomous organization (DAO) is akin to an on-chain government where participants are able to cast votes on proposals using some governance token. Various consensus mechanisms may be implemented in order to determine whether a proposal will pass and if that happens then the DAO will begin to operate under the rules of the new proposal. In this implementation the user deposits governance tokens and receives some number of votes that can be cast and recast on different proposals. They can vote in favour or against proposals and they can transform their votes back into the governance tokens if they wish to withdraw.

More information can be found in the [specification](./SPECIFICATION.md).

### Current state of the application

- The smart contract is mostly complete for the basic implementation. There are some issues that need to be worked out but a UI can be started
- The user interface does not currently exist

## Project Structure

The project consists of a smart contract and a user interface which the user can interact with.

<!--Only show most important files e.g. script to run, build etc.-->

```
dao-voting/
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

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/dao-voting/<you are here>`

There are two commands required to run the tests

1. Build the asset used for depositing into the dao-voting
   
   ```bash
   forc build --path tests/artifacts/gov_token 
   ```

2. Run the tests

   ```bash
   forc build
cargo test
   ```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the dao-voting.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
