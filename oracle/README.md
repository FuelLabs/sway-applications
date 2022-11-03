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

The node can be started by executing the following steps:

1. Change ino the `Oracle` directory

```bash
cd <path>/sway-applications/oracle/<you are here>
```

2. Copy and paste the `.env.example` file into a new file called `.env`

```bash
cp packages/node/.env.example packages/node.env
```

| Name               | Description |
|--------------------|-------------|
| API_URL            | The URL the node uses to fetch the latest price for the asset tracked by the oracle. |
| ORACLE_CONTRACT_ID | Deterministic contract id of the oracle contract which is deployed in a later step |
| WALLET_SECRET      | Private key of the first deterministic wallet provided by the fuels-rs sdk.  It is also the private key corresponding to the `owner` address specified in the oracle contract's `Forc.toml`.  It is configured to have the maximum amount of the `BASE_ASSET` by the local `fuel-core` instance spun up in the step 4. |
| FUEL_PROVIDER_URL  | Fuel-core network url normally set as http://localhost:4000/graphql for development |

3. In the newly copied `.env` file there is a variable `API_URL` which ends with `<your api key here>`.  This section should be replace with your API key.

    **_Note:_** You do not need an api key to run any tests

4. Start a local `fuel-core` instance

```bash
fuel-core run --chain packages/node/.chainConfig.json
```

This will allow us to deploy the oracle contract.  It configures the local `fuel-core` instance with the information specified in `.chainConfig.json`.  It also initializes the wallet specified in `.chainConfig.json` with the maximum amount of the `BASE_ASSET`.

5. Deploy the Oracle contract

```bash
forc-deploy --path packages/contract --url localhost:4000 --unsigned
```

This will allow the node to interact with the oracle contract deployed to our local `fuel-core` instance.

6. Start the Oracle node

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
