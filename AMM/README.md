<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/amm_logo-dark_theme.png">
        <img alt="automated market maker logo" width="600px" src=".docs/amm_logo-light_theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.33.1" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.33.1-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.15.3" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.15.3-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.36.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.36.0-blue" />
    </a>
</p>

## Overview

An *automated market maker (AMM)* is a type of decentralized exchange protocol that determines asset prices algorithmically through a conservation function. Trades on an AMM take place between the user and the contract, rather than between two users. The liquidity pool of assets in an AMM is supplied by the users. Providing liquidity is incentivized via liquidity miner rewards. 

This application supports
- Depositing assets
- Withdrawing assets
- Adding liquidity using deposited assets
- Removing liquidity
- Swapping assets

The contracts are designed to
- Support liquidity pools that consist of two assets
- Use a conservation function which keeps the total liquidity at a constant ratio
    - $price_{asset\ A} * price_{asset\ B} = total\ liquidity$
- Provide a liquidity miner fee of $\frac1{333} \approx 0.3\%$

	> **NOTE** The miner fee can be modified per asset pair

## Project structure

```sh
AMM
├── project
│   ├── contracts
│   │   ├── AMM-contract
│   │   └── exchange-contract
│   ├── libraries
│   │   └── src/interface.sw
│   ├── scripts
│   │   ├── atomic-add-liquidity
│   │   ├── swap-exact-input
│   │   └── swap-exact-output
|   ├── test-utils
|   |   └── src/lib.rs
|   ├── README.md
│   └── SPECIFICATION.md
├── ui
│   ├── README.md
│   └── SPECIFICATION.md
└─── README.md
```

All contracts and scripts have the structure:

```
contract or script/
├── src/main.sw
└── tests/harness.rs
```

## Running the project

### User interface

TODO: The user interface does not currently exist therefore its [README.md](ui/README.md) and [SPECIFICATION.md](ui/SPECIFICATION.md) are empty.

### Project

In order to run the subsequent commands change into the following directory `/path/to/AMM/project/<here>`.

#### Program compilation

```bash
forc build
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test
```
