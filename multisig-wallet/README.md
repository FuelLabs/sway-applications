<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/multi-signature-logo-dark-theme.png">
        <img alt="multisig logo" width="400px" src=".docs/multi-signature-logo-light-theme.png">
    </picture>
</p>

## Overview

A multi-signature wallet is a wallet that has multiple owners. In order to execute a transaction, a sufficient number of owners need to sign a Tx. Moreover, this implementation uses weighted owners which means that certain owners may have more "votes" when it comes to increasing the number of approvals in order to surpass the minimum threshold for execution.

More information can be found in the [specification](./SPECIFICATION.md).

### Current state of the application

- The smart contract is under development and is not ready for integration into a user interface
- The user interface does not currently exist
- Spec needs to be added
- Tests are being written as functionality is added therefore skip step 1 of running the tests

## Project Structure

The project consists of a smart contract and a user interface which the user can interact with.

<!--Only show most important files e.g. script to run, build etc.-->

```
multisig-wallet/
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

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/multisig-wallet/<you are here>`

Use the following command to run the tests

1. Run the tests

   ```bash
   forc build
   cargo test
   ```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the multisig-wallet.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
