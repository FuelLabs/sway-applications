<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/nft-logo_white.png">
        <img alt="light theme" src=".docs/nft-logo_black.png">
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

A non-fungible token (NFT) is a unique asset that has a maximum supply of one. On the Fuel Network, all NFTs are [Native Assets](https://docs.fuel.network/docs/sway/blockchain-development/native_assets). They are commonly associated with artwork / collectibles however there are many greater utilities beyond that which have yet to be written for the Fuel Network.

In this barebones NFT example project, there are a maximum of 100,000 NFTs that may be minted. Each NFT may contain any metadata the user desires to store. 

## Standards Implementations

The project implements and follows the [SRC-20; Native Asset](https://github.com/FuelLabs/sway-standards/blob/master/SRCs/src-20.md), [SRC-3; Mint and Burn](https://github.com/FuelLabs/sway-standards/blob/master/SRCs/src-3.md), and [SRC-7; Metadata](https://github.com/FuelLabs/sway-standards/blob/master/SRCs/src-7.md) standards. It also uses the [Native Asset Library](https://fuellabs.github.io/sway-libs/book/asset/index.html) to implement the basic functionality behind the standards.  

### SRC-20

The [SRC-20](https://github.com/FuelLabs/sway-standards/blob/master/SRCs/src-20.md) standard requires that there is a maximum number of one coin per NFT asset. It also states that the decimals must be `0u8` for any NFT. This project conforms to both these restrictions and thus can be deemed an NFT on the Fuel Network. 

Set functions for name and symbol have been provided to the user. While traditionally name and symbol are written into the contract rather than storage, this contract is open to mint new types of assets. This means that every NFT minted by this contract may contain a different name and symbol. 

The [SRC-20](https://github.com/FuelLabs/sway-standards/blob/master/SRCs/src-20.md) ABI defined below has also been implemented.

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

The [SRC-3](https://github.com/FuelLabs/sway-standards/blob/master/SRCs/src-3.md) standard defines the ABI for minting and burning. This has been properly implemented.

```sway
abi SRC3 {
    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: SubId, amount: u64);
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64);
}
```

### SRC-7

The [SRC-7](https://github.com/FuelLabs/sway-standards/blob/master/SRCs/src-7.md) standard defines the ABI for retrieving metadata. This has been properly implemented. 

A set function that uses storage has been provided to allow the user to set their own desired metadata. There is no limit or restrictions to what and the amount of metadata an asset may have.

```sway
abi SRC7 {
    #[storage(read)]
    fn metadata(asset: AssetId, key: String) -> Option<Metadata>;
}
```

## Project structure

The project consists of a smart contract.

<!--Only show most important files e.g. script to run, build etc.-->

```sh
NFT
├── NFT-contract
│   └──src/main.sw
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User interface

TODO: The user interface does not currently exist.

### Project

In order to run the subsequent commands change into the following directory `/path/to/NFT/<here>`.

#### Program compilation

```bash
forc build
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
forc test 
```
