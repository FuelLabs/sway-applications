<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/name-registry-logo-dark-theme.png">
        <img alt="SwayApps Fundraiser Logo" width="400px" src=".docs/name-registry-logo-light-theme.png">
    </picture>
</p>

## Overview

This is a name registry app built in Sway, which allows users to use simple names instead of long addresses to recieve or send payments, making it easy to send money without copy pasting addresses. Anyone can register a name for the price of 1 unit of any asset per 100 seconds. The name will resolve to any Identity that the owner sets.

In this implementation the price is paid in the base asset on the Fuel network. More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of a smart contract and a user interface which the user can interact with.

<!--Only show most important files e.g. script to run, build etc.-->

```
name-registry/
├── contract/
|    └── src/main.sw
|    └── tests/harness.rs
├── frontend/
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/name-registry/<you are here>`

Run the tests
   ```bash
   forc test
   ```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the fundraiser.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
