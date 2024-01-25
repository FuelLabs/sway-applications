<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/native-asset-logo-dark-theme.png">
        <img alt="light theme" src=".docs/native-asset-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.49.1" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.49.1-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.22.0" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.22.0-yellow" />
    </a>
</p>

## Overview

A fungible asset is a [Native Asset](https://docs.fuel.network/docs/sway/blockchain-development/native_assets) on the Fuel Network.

In this project, there are a maximum of 100,000,000 coins for each asset that may be minted. There is no limit on the total individual assets a user may mint.

## Standards Implementations

The project implements and follows the [SRC-20; Native Asset](https://github.com/FuelLabs/sway-standards/tree/master/standards/src20-native-asset) and [SRC-3; Mint and Burn](https://github.com/FuelLabs/sway-standards/tree/master/standards/src3-mint-burn) standards. It also uses the [Native Asset Library](https://github.com/FuelLabs/sway-libs/blob/master/libs/native_assets) to implement the basic functionality behind the standards.  

### SRC-20

Set functions for name, symbol, and decimals have been provided to the user. While traditionally name, symbol, and decimals are written into the contract rather than storage, this contract is open to mint new types of assets. This means that every asset minted by this contract may contain a different name and symbol. 

The [SRC-20](https://github.com/FuelLabs/sway-standards/tree/master/standards/src20-native-asset) ABI defined below has also been implemented.

```sway
abi SRC20 {
    #[storage(read)]
    fn total_assets() -> u64;
    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64>;
    #[storage(read)]
    fn name(asset: AssetId) -> Option<String>;
    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String>;
    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8>;
}
```

### SRC-3

The [SRC-3](https://github.com/FuelLabs/sway-standards/tree/master/standards/src_3) standard defines the ABI for minting and burning. This has been properly implemented.

```sway
abi SRC3 {
    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: SubId, amount: u64);
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64);
}
```

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
native-asset
├── project
│   ├── contracts
│   │   └── native-asset-contract
│   │       └──src/main.sw
│   └── SPECIFICATION.md
├── ui
│   └── SPECIFICATION.md
└── README.md
```

## Running the project

### User interface

TODO: The user interface does not currently exist therefore its [SPECIFICATION.md](ui/SPECIFICATION.md) is empty.

### Project

In order to run the subsequent commands change into the following directory `/path/to/native-asset/project/<here>`.

#### Program compilation

```bash
forc build
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
forc test 
```
