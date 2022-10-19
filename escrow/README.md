<picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/escrow-logo-dark-theme.png">
        <img alt="escrow logo" width="400px" src=".docs/escrow-logo-light-theme.png">
</picture>
</p>

<h2>📝&nbsp; Table of Contents</h2>

An escrow is a neutral third party that holds an asset on behalf of two parties until a transaction has occurred. Once the transaction has taken place the escrow can be resolved and the assets will be transferred as dictated by the contract. This escrow application handles the transaction between an on-chain and off-chain asset.

The off-chain asset can be anything the user desires. For example, some currency which will be used as a payment for a product or perhaps a deed to an estate. Within the blockchain world it all comes down to 1's and 0's, however, those assets can be tokenized and logic can be coded into a contract to handle different cases. But in the end, it still comes down to good faith outside of the blockchain to carry out the obligation.

For this application, the on-chain asset can be any native asset on the Fuel Network and the transaction is considered to be between a buyer and a seller. At the moment, and for the foreseeable future, there is another party (the arbiter) in case there is a dispute that the buyer and seller cannot resolve amongst themselves. More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of a smart contract and a user interface which the user can interact with.

<!--Only show most important files e.g. script to run, build etc.-->

```
escrow/
├── contract/
|    └── src/main.sw
|    └── tests/harness.rs
├── frontend/
|    └── Directories & files
├── README.md
└── SPECIFICATION.md
```

## 🖥️ Running The Project

### Requirements

This project includes both frontend and contracts. To begin, install dependencies:

- [Node.js v16.15.0 or latest stable](https://nodejs.org/en/). We recommend using [nvm](https://github.com/nvm-sh/nvm) to install.
- [PNPM v7.1.7 or latest stable](https://pnpm.io/installation/)
- [Rust toolchain v0.16.0 or latest `stable`](https://www.rust-lang.org/tools/install)
- [Forc v0.19.2](https://fuellabs.github.io/sway/v0.19.2/introduction/installation.html#installing-from-pre-compiled-binaries)
- [Docker v0.8.2 or latest stable](https://docs.docker.com/get-docker/)
- [Docker Compose v2.6.0 or latest stable](https://docs.docker.com/get-docker/)

### 📚 - Getting the Repository

1. Visit the [Sway-Applications](https://github.com/FuelLabs/sway-applications) repo and fork the project.
2. Then clone your forked copy to your local machine and get to work.

```sh
git clone https://github.com/FuelLabs/sway-applications
cd sway-applications/escrow
```

### 📦 - Install Dependencies

```sh
pnpm install
```

### 📒 - Run Local Node

In this step, we are going to;

- Setup `escrow-scripts`
- launch a local `fuel-core` node
- Build and deploy the Escrow contracts
- Generate and seed 10 wallets for frontend use

```sh
pnpm services:setup
```

### 💻 - Run Web App

Start a local development frontend. After running the command below you can open [http://localhost:3000](http://localhost:3000) in your browser to view the frontend.

```sh
pnpm dev
```

## 📗 Project Overview
This section has a brief description of each directory.  More details can be found inside of each package by clicking the links.

 - [packages/frontend](./packages/frontend/) Frontend Escrow application
 - [packages/contract](./packages/contracts/) 🌴 Sway contracts
 - [packages/scripts](./packages/scripts/) Escrow scripts CLI
 - [packages/config](./packages/config/) Build configuration
 - [docker](./docker/) Fuel core docker configuration

## 🧰 Useful Scripts

To make life easier we added as many useful scripts as possible to our [package.json](../package.json). These are some of the most used during development:

```sh
pnpm <command name>
```

| Script             | Description                                                                                                                              |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `dev`              | Run development server for the WebApp [packages/app](../packages/app/).                                                                  |
| `contracts`        | Build, generate types, deploy [packages/contracts](../packages/contracts). It should be used when editing contracts.                     |
| `contracts:build`  | Build and generate types [packages/contracts](../packages/contracts).                                                                    |
| `contracts:deploy` | Deploy the current binaries.                                                                                                             |
| `scripts:setup`    | Setup [escrow-scripts](../packages/scripts/) used to build and deploy contracts and generate types.                                      |
| `services:clean`   | Stop and remove all development containers that are running locally.                                                                     |
| `services:run`     | Run the local network with `fuel-core`                                                                                                   |
| `services:setup`   | Run the local network, setup `escrow-scripts`, build and deploy contracts normally used on the first run, and generate and seed wallets. |
| `services:reset`   | Runs `services:clean` then `services:setup`                                                                                              |

> Other scripts can be found in [package.json](../package.json).

## Testing the Program

To test the contracts go to `/escrow/packages/contracts/escrow`

There are two commands required to run the contract tests:

1. Build the asset used for depositing into the escrow
   
    ```bash
    forc build --path tests/artifacts/asset/
    ```

2. Run the tests
    
    ```bash
    forc test
    ```

To test the frontend go to back to the root of the project i.e. `/escrow`

There are two commands required to run the frontend tests:

1. Run the frontend required for testing
    
    ```bash
    pnpm dev
    ```

2. Run the tests

    ```bash
    pnpm test
    ```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the escrow.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
