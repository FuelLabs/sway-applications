<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/oracle-logo-dark-theme.png">
        <img alt="oracle logo" width="400px" src=".docs/oracle-logo-light-theme.png">
    </picture>
</p>

## Overview

Oracles provide blockchain applications access to off-chain information such as asset prices, and verifiable random numbers.  Oracles allow blockchain applications to react to real-world events such as a price drop in collateral or the winner of a sporting event.  Oracles typically rely on a trusted off-chain node to provide them with the correct data.  This example oracle provides price data about a specific asset, and assumes a decimal precision of 1e9.

More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of an oracle smart contract and an oracle node which interacts with the oracle.

<!--Only show most important files e.g. script to run, build etc.-->

```
oracle/
├── packages/
|    └── contract/
|       └── src/main.sw
|       └── tests/harness.rs
|    └── node/
|       └── Directories & files
├── frontend/
|    └── Directories & files
└── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: need UI for this to be relevant

### Node
In order to run the Oracle node make sure you are in the root of this project i.e `/path/to/oracle/<you are here>`

Copy the `.env.example` file into a new file called `.env`.  Then insert your api key into the `API_URL`

**_Note:_** You do not need an api key to run any tests


Then start a local fuel-core instance
```bash
fuel-core run --chain packages/node/.chainConfig.json
```

Then deploy an instance of the Oracle contract
```bash
forc deploy --path packages/contract --url localhost:4000 --unsigned
```

Finally start the Oracle node
```bash
cargo run
```

### Tests
In order to run the tests make sure you are in the root of this project i.e. `/path/to/oracle/<you are here>`

Run the tests with the following command.

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
