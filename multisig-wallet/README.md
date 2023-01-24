<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/multi-signature-logo-dark-theme.png">
        <img alt="multisig logo" width="400px" src=".docs/multi-signature-logo-light-theme.png">
    </picture>
</p>

## Overview

A multi-signature wallet is a wallet that has multiple owners. In order to execute a transaction, a sufficient number of owners need to sign a transaction. This implementation uses weighted owners which means that certain owners may have more "votes" when it comes to increasing the number of approvals in order to surpass the minimum threshold for execution. This implementation allows owners to be both Fuel and EVM addresses; by additionally supporting signatures over [EIP-191](https://eips.ethereum.org/EIPS/eip-191) formatted messages.

More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```
multisig-wallet/
├── project/
|   └── multisig-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/multisig-wallet/<you are here>`

Build the contract:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the multisig-wallet.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
