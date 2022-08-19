<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/escrow-logo-dark-theme.png">
        <img alt="escrow logo" width="400px" src=".docs/escrow-logo-light-theme.png">
    </picture>
</p>

## Overview

Oracles provide blockchain applications access to off-chain information such as asset prices, and verifiable random numbers.  Oracles allow blockchain applications to react to real-world events such as a price drop in collateral or the winner of a sporting event.  Oracles typically rely on a trusted off-chain node to provide them with the correct data.

## Project Structure

The project consists of an oracle smart contract and an oracle node which interacts with the oracle.

<!--Only show most important files e.g. script to run, build etc.-->

```
oracle/
├── contract/
|    └── src/main.sw
|    └── tests/harness.rs
├── node/
|    └── Directories & files
└── README.md
```

## Running the project

### User Interface

TODO: need UI for this to be relevant

### Tests
In order to run the tests make sure you are in the root of this project i.e. `/path/to/oracle/<you are here>`

Run the tests with the following command.

```bash
forc test
```

## Contributing

Check [CONTRIBUTING.md](../CONTRIBUTING.md) for more info!
