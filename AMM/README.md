<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/amm_logo-dark_theme.png">
        <img alt="automated market maker logo" width="600px" src=".docs/amm_logo-light_theme.png">
    </picture>
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

## Project Structure

```
AMM/
├── project/
|   ├── contracts/
|   |   ├── AMM-contract/
|   |   └── exchange-contract/
|   ├── scripts/
|   |   ├── atomic-add-liquidity/
|   |   ├── swap-exact-input/
|   |   └── swap-exact-output/
|   ├── libraries/
|   |   └── src/interface.sw
|   └── test-utils/
|       └── src/lib.rs
├── README.md
└── SPECIFICATION.md
```

All contracts and scripts have the structure:

```
contract or script/
├── src/main.sw
└── tests/harness.rs
```

## Running the project

### User Interface

TODO: UI is to be added.

### Tests

In order to run the tests make sure that you are in the root of this project `/path/to/AMM/<you are here>`

Build the contracts:

```bash
forc build
```

Run the tests:

```bash
cargo test
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the AMM.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
