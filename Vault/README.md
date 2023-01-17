<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/vault-logo-dark-theme.png">
        <img alt="SwayApps Vault Logo" width="400px" src=".docs/vault-logo-light-theme.png">
    </picture>
</p>

## Overview

TODO: A summary of the application should be written here.

More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```
Vault/
├── project/
|   └── vault-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/Vault/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the vault.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!