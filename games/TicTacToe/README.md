<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/tictactoe-logo-dark-theme.png">
        <img alt="SwayApps TicTacToe Logo" width="400px" src=".docs/tictactoe-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.33.1" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.33.1-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.15.3" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.15.3-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.34.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.34.0-blue" />
    </a>
</p>

## Overview

An on-chain TicTacToe game, where two players compete to align 3 markers in a row. The game consists of a 3x3 grid.
The game has been won if three markers were aligned in a row. Otherwise, it's a draw.

More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of a smart contract.

```
TicTacToe/
├── project/
|   └── tictactoe-contract/
|       ├── src/main.sw
|       └── tests/harness.rs
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/TicTacToe/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the tictactoe.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!