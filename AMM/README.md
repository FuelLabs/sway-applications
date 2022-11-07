<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/amm_logo-dark_theme.png">
        <img alt="automated market maker logo" width="600px" src=".docs/amm_logo-light_theme.png">
    </picture>
</p>

## Overview

An *automated market maker (AMM)* is a type of decentralized exchange protocol that determines asset prices algorithmically through a conservation function. Trades on an AMM take place between the user and the contract, rather than between two users. The liquidity pool of assets in an AMM is supplied by the users. Providing liquidity is incentivized via liquidity miner rewards. This application allows users to deposit assets that can be used to add liquidity or be withdrawn without adding liquidity, adding and removing liquidity and swapping assets.

This application contains an AMM contract that has: 
- Liquidity pools of two assets each,
- The conservation function $price_{asset\ A} * price_{asset\ B} = total\ liquidity$, which means that swap operations keep the total liquidity constant,
- Liquidity miner fee of $\frac1{333} \approx 3\%$.

    > **NOTE** This fee can be modified as desired per asset pair.

### Current state of the application

User interface is to be added.

Functionality of this application is being enhanced with two Sway scripts (in progress) that allows:
1) Swapping assets along a route,
2) Depositing and adding liquidity atomically.

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
|    └── src/interfaces.sw
└── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI is to be added.

### Tests

To run the tests for either contract, you should first build the libraries if you have not already built the contracts. 

For this, make sure that you are in the root of this project, i.e., `/path/to/AMM/<you are here>`.

Build the `libraries` project:
```bash
forc build --path libraries/
```

- In order to run the AMM contract tests, move to the root of the AMM contract, i.e., `/path/to/AMM/contracts/AMM/<you are here>`. Make sure that you have built the contract using the command `forc build`.

    1. Build the contract used for testing against malicious implementations of the exchange contract:
        ```bash
        forc build --path ../exchange/tests/artifacts/malicious_implementation/
        ```
    2. Run the tests:
        ```bash
        cargo test
        ```
- In order to run the exchange contract tests, move to the root of the exchange contract, i.e., `/path/to/AMM/contracts/exchange/<you are here>`. Make sure that you have built the contract using the command `forc build`. 

    Run the tests:
    ```bash
    cargo test
    ```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the AMM.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!
