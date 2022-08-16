<picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/escrow-logo-dark-theme.png">
        <img alt="escrow logo" width="400px" src=".docs/escrow-logo-light-theme.png">
</picture>
</p>

<h2>📝&nbsp; Table of Contents</h2>

- [🙋🏻&nbsp; Getting Started](#-getting-started)
- [🖥️ Running The Project](#running-the-project)
- [📜&nbsp; License](#-license)

---

## 🙋🏻&nbsp; Getting Started

First, go through the setup process in [CONTRIBUTING.md](../../../CONTRIBUTING.md)

## Project Structure

TODO: need UI for this to be relevant

<!--Only show most important files e.g. script to run, build etc.-->

```
escrow/
├── contract/
|    └── src/main.sw
|    └── tests/harness.rs
├── frontend/
|    └── Directories & files
└── README.md
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

### 🧰 Useful Scripts

To make life easier we added as many useful scripts as possible to our [package.json](../package.json). These are some of the most used during development:

```sh
pnpm <command name>
```

| Script             | Description                                                                                                          |
| ------------------ | -------------------------------------------------------------------------------------------------------------------- |
| `dev`              | Run development server for the WebApp [packages/app](../packages/app/).                                              |
| `contracts`        | Build, generate types, deploy [packages/contracts](../packages/contracts). It should be used when editing contracts. |
| `contracts:build`  | Build and generate types [packages/contracts](../packages/contracts).                                                |
| `contracts:deploy` | Deploy the current binaries.                                                                                         |
| `scripts:setup`    | Setup [escrow-scripts](../packages/scripts/) used to build and deploy contracts and generate types.                |
| `services:clean`   | Stop and remove all development containers that are running locally.                                                 |
| `services:run`     | Run the local network with `fuel-core`                                                      |
| `services:setup`   | Run the local network, setup `escrow-scripts`, build and deploy contracts normally used on the first run, and generate and seed wallets.       |
| `services:reset`   | Runs `services:clean` then `services:setup`       |

> Other scripts can be found in [package.json](../package.json). (some of these commands don't currently work as some features are still in development)

## Testing the Program

Insde of `/contracts/escrow` folder test the contracts:

```bash
forc test
```

## Contributing

Check [CONTRIBUTING.md](../CONTRIBUTING.md) for more info!

## 📜&nbsp; License

The primary license for this repo is `Apache 2.0`, see [`LICENSE`](./LICENSE).
