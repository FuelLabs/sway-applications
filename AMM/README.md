<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/amm_logo-dark_theme.png">
        <img alt="automated market maker logo" width="600px" src=".docs/amm_logo-light_theme.png">
    </picture>
</p>

## Overview

An *automated market maker (AMM)* is a type of decentralized exchange protocol that determines asset prices algorithmically through a conservation function. Trades on an AMM take place between the user and the contract, rather than between two users. The liquidity pool of assets in an AMM is supplied by the users. Providing liquidity is incentivized via liquidity miner rewards. 

This application supports:
- Depositing assets that can be
    - Used to add liquidity
    - Withdrawn without adding liquidity
- Adding liquidity
- Removing liquidity
- Swapping assets

The AMM and exchange contracts in this application is designed so that:
- Liquidity pools consist of two assets each
- The conservation function used to keep the total liquidity at a constant ratio when exchanging assets is
    - $price_{asset\ A} * price_{asset\ B} = total\ liquidity$
- Liquidity miner fee is $\frac1{333} \approx 3\%$

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

To run the tests for either contract, firstly make sure that you are in the root of this application, i.e., `/path/to/AMM/<you are here>`.

- In order to run the AMM contract tests, change directory to the root of the AMM contract project, i.e., `/path/to/AMM/contracts/AMM/<you are here>`:
    ```bash
    cd contracts/AMM
    ```
    1. Build the AMM contract:
        ```bash
        forc build
        ```
    2. Build the contract used for testing against malicious implementations of the exchange contract:
        ```bash
        forc build --path ../exchange/tests/artifacts/malicious_implementation/
        ```
    3. Run the tests:
        ```bash
        cargo test
        ```
- In order to run the exchange contract tests, change directory to the root of the exchange contract project, i.e., `/path/to/AMM/contracts/exchange/<you are here>` from the root of the AMM contract project:
    ```bash
    cd ../contracts/exchange
    ```
    1. Build the exchange contract:
        ```bash
        forc build
        ```
    2. Run the tests:
        ```bash
        cargo test
        ```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the AMM.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
