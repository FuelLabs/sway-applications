<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/tictactoe-logo-dark-theme.png">
        <img alt="SwayApps TicTacToe Logo" width="400px" src=".docs/tictactoe-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.42.1" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.42.1-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.18.3" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.18.3-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.43.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.43.0-blue" />
    </a>
</p>

## Overview

An on-chain TicTacToe game, where two players compete to align 3 markers in a row. The game consists of a 3x3 grid.
The game has been won if three markers were aligned in a row. Otherwise, it's a draw.

More information can be found in the [specification](./project/SPECIFICATION.md).

## Project structure

The project consists of a smart contract.

```sh
TicTacToe
├── project
│   ├── contracts
│   │   └── tictactoe-contract
│   │       ├── src/main.sw
│   │       └── tests/harness.rs
│   ├── README.md
│   └── SPECIFICATION.md
├── ui
│   ├── README.md
│   └── SPECIFICATION.md
└── README.md
```

## Running the project

### User interface

TODO: The user interface does not currently exist therefore its [README.md](ui/README.md) and [SPECIFICATION.md](ui/SPECIFICATION.md) are empty.

### Project

In order to run the subsequent commands change into the following directory `/path/to/TicTacToe/project/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
