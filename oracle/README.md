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

### Oracle Node and Contract

The project can be started by executing the following steps:

1. Change into the `Oracle` directory.

    ```bash
    cd <path>/sway-applications/oracle/<you are here>
    ```

2. Copy and paste the `.env.example` file into a new file called `.env`.

    ```bash
    cp packages/node/.env.example packages/node/.env
    ```

3. In the newly copied `.env` file there is a variable `API_URL` which ends with `<your api key here>`.  This section should be replaced with your API key.  You can read more about this project's environment variables [here](#environment-variables)

4. Start a local `fuel-core` instance.

    ```bash
    fuel-core run --chain packages/node/.chainConfig.json
    ```

    This spins up and configures the local `fuel-core` instance with the variables specified in [`.chainConfig.json`](./packages/node/.chainConfig.json).

5. Deploy the Oracle contract.

    ```bash
    forc-deploy --path packages/contract --url localhost:4000 --unsigned
    ```

    This will allow the node to interact with the oracle contract deployed to our local `fuel-core` instance.

    Note: Double check that the contract ID is the ID in the `.env` file.

6. Start the Oracle node.

    ```bash
    cargo run
    ```

### Environment Variables

| Name               | Description |
|--------------------|-------------|
| API_URL            | The URL the node uses to fetch the latest price for the asset tracked by the oracle. This oracle node relies on an external 3rd-party service to get price information to provide to the oracle contract.  We do not endorse this service neither are we affiliated with them in any way.  We only use the service for demonstration purposes.  If you wish to run the node you can sign-up for a free api key [here](https://www.cryptocompare.com/).  If you wish to use another pricing api service feel free to replace `API_URL` entirely. |
| ORACLE_CONTRACT_ID | Deterministic contract id of the oracle contract deployed in step 5. |
| WALLET_SECRET      | Private key of the first deterministic wallet provided by the [fuels-rs](https://github.com/FuelLabs/fuels-rs) sdk.  This private key correspondes to the `owner` address specified in the oracle contract's [`Forc.toml`](./packages/contract/Forc.toml).  This address is also configured in step 4 to have the maximum amount of the [BASE_ASSET](https://github.com/FuelLabs/sway/blob/master/sway-lib-std/src/constants.sw). |
| FUEL_PROVIDER_URL  | Fuel-core network url normally set as http://localhost:4000/graphql for development. |

### Tests
In order to run the tests make sure you are in the root of this project i.e. `/<path>/sway-applications/oracle/<you are here>`

Run the tests with the following command.

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
