<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/english-auction_dark.png">
        <img alt="light theme" src=".docs/english-auction_light.png">
    </picture>
</p>

## Overview

An English Auction is where a seller auctions off an asset with an initial price and a reserve price. Users will then begin bidding on the asset until the bidding period has ended or the reserve has been met. Upon completion, users will withdraw either their original deposits or their newly purchased assets depending on the outcome.

The English Auction application implements this idea in a decentralized manner without the need for a 3rd party and with strong settlement assurances. The application has been designed to utilize native assets and NFTs enabling users to auction off native assets / NFTs and place bids using native assets / NFTs.

More information can be found in the [specification](./SPECIFICATION.md).

## Repository Structure

The project consists of a smart contract.

```
english-auction/
├── docker/
├── packages
|   └── app/
|   └── config/
|   └── contracts/project/auction-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
|   └── scripts/
├── README.md
└── SPECIFICATION.md
```

## Running the project

### Requirements

- [Node.js v16.15.0 or latest stable](https://nodejs.org/en/). We recommend using [nvm](https://github.com/nvm-sh/nvm) to install.
- [PNPM v7.1.7 or latest stable](https://pnpm.io/installation/)
- [Docker v0.8.2 or latest stable](https://docs.docker.com/get-docker/)
- [Docker Compose v2.6.0 or latest stable](https://docs.docker.com/get-docker/)

### User Interface

To run the frontend locally first make sure that your are in the root of this project directory i.e `/path/to/english-auction/`

Install dependencies

```bash
pnpm install
```

In this step we are going to:

- Launch a local `fuel-core` node
- Setup `english-auction-scripts`
- Build and deploy the english auction contracts
- Initialize the NFT contracts

```bash
pnpm services:setup
```

Ensure that `VITE_FUEL_PROVIDER_URL` is set to `http://localhost:4001/graphql` inside of the newly created `.env` file.

Run web app

```bash
pnpm dev
```

You can now interact with the web app on [http://localhost:3000](http://localhost:3000)

### Project Overview

This section has a brief description of each directory. More details can be found inside each package, by clicking on the links.

- [packages/app](../packages/app/) Frontend English Auction application
- [packages/contracts](../packages/contracts/) English Auction 🌴 Sway contracts
- [packages/scripts](../packages/scripts/) English Auction scripts CLI
- [packages/config](../packages/config/) Build configurations
- [docker](../docker/) Network configurations

### 🧰 Useful Scripts

To make life easier we added as many useful scripts as possible to our [package.json](../package.json). These are some of the most used during development:

```sh
pnpm <command name>
```

| Script             | Description                                                                                                                     |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| `dev`              | Run development server for the WebApp [packages/app](../packages/app/).                                                         |
| `contracts`        | Build, generate types, and deploy [packages/contracts](../packages/contracts). It should be used when editing contracts.        |
| `contracts:build`  | Build and generate types [packages/contracts](../packages/contracts).                                                           |
| `contracts:deploy` | Deploy the current contract binaries.                                                                                           |
| `scripts:setup`    | Setup the [english-auction-scripts](../packages/scripts/) package which is used to build, deploy contracts, and generate types. |
| `services:clean`   | Stop and remove all development containers that are running locally.                                                            |
| `services:run`     | Run the local network with `fuel-core`.                                                                                         |
| `services:setup`   | Run the local network, setup `english-auction-scripts` and build and deploy contracts normally used on the first run.           |

> Other scripts can be found in [package.json](../package.json).

### User Interface E2E Tests

In order to run the user interface e2e tests make sure that you are in the root of this directory i.e `/path/to/english-auction/`

Run a local node and setup contracts in test env

```bash
pnpm services:setup-test
```

Run test

```bash
pnpm test
```

> **Note**
> In order to run the tests `VITE_FUEL_PROVIDER_URL` must be set to `http://localhost:4000/graphql` inside of `.env.test`

> **Note**
> After you run the tests once they will not all pass again until you `pnpm services:reset-test`

### Rust Unit Tests

In order to run the rust unit tests make sure that you are in this directory `/english-auction/packages/contracts/english-auction/project/auction-contract/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the english-auction.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
