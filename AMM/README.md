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
- Provide a liquidity miner fee of $\frac1{333} \approx 3\%$

	> **NOTE** The miner fee can be modified per asset pair

## Project Structure

```
AMM/
├── contracts/
|    └── AMM/
|         ├── src/main.sw
|         └── tests/harness.rs
|    └── exchange/
|         ├── src/main.sw
|         └── tests/harness.rs
├── libraries/
|    └── src/interface.sw
└── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI is to be added.

### Tests

To run the tests for either contract follow the instructions below.

Change into the following directory:

```bash
cd /<path>/sway-applications/AMM/contracts
```

#### AMM

Build the contract:

```bash
forc build --path ./AMM
```

Build the exchange contract:

```bash
forc build --path ./exchange
```

Build the malicious exchange contract:

```bash
forc build --path ./exchange/tests/artifacts/malicious_implementation
```

Run the tests:

```bash
cargo test --manifest-path ./AMM/Cargo.toml
```

#### Exchange

Build the contract:

```bash
forc build --path ./exchange
```

Run the tests:

```bash
cargo test --manifest-path ./exchange/Cargo.toml
```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the AMM.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
