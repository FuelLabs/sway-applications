<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/name-registry-logo-dark-theme.png">
        <img alt="SwayApps Fundraiser Logo" width="400px" src=".docs/name-registry-logo-light-theme.png">
    </picture>
</p>

## Overview

The name registry application allows users to use human readable names instead of addresses to send and receive payments, making it easier to transfer cryptocurrency. 

A name can be registered for the price of 1 unit of any asset per 100 seconds and it resolves to any Identity the owner sets.
In this implementation the price is paid in the base asset on the Fuel network.

Both the asset and the price per 100 seconds are configuration time constants so can be easily changed to different values.

More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of a smart contract and a user interface which the user can interact with.

<!--Only show most important files e.g. script to run, build etc.-->

```
name-registry/
├── project/
|   └── registry-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

Make sure that you are in the root of the name registry project i.e. `/path/to/name-registry/<you are here>`

Build the contract:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the name-registry.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
