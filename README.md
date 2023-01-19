<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/sway-apps-logo-dark-theme.png">
        <img alt="SwayApps logo" width="400px" src=".docs/sway-apps-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://github.com/FuelLabs/sway-applications/actions/workflows/ci.yml" alt="CI">
        <img src="https://github.com/FuelLabs/sway-applications/actions/workflows/ci.yml/badge.svg" />
    </a>
    <a href="https://crates.io/crates/forc/0.32.2" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.32.2-orange" />
    </a>
    <a href="./LICENSE" alt="forc">
        <img src="https://img.shields.io/github/license/FuelLabs/sway-applications" />
    </a>
    <a href="https://discord.gg/xfpK4Pe">
        <img src="https://img.shields.io/discord/732892373507375164?color=6A7EC2&logo=discord&logoColor=ffffff&labelColor=6A7EC2&label=Discord" />
    </a>
</p>

## Overview

The purpose of this repository is to contain end-to-end applications that are written in Sway in order to demonstrate what can be built.

This means that a project will generally consist of a Sway contract and a user interface in order to interact with the contract however that is not a hard rule.

> **Note**
> Sway is a language under heavy development therefore the applications may not be the most ergonomic. Over time they should receive updates / improvements in order to demonstrate how Sway can be used in real use cases.

## Repository Structure

Each project within this repository is independent of the other projects and thus every project has its own directory with all of the files required to make it work.

That being said they are all under the same [CI](.github/workflows/ci.yml) so any updates to a project must make sure that the other projects continue to pass.

The following is a visual sample of how the repository is structured.

```
sway-applications/
├── Project-1
|    └── P-1 Directories & files
├── Project-2
|    └── P-2 Directories & files
├── LICENSE
└── README.md
```

### Projects

- [Airdrop](./airdrop/) is a token distribution program where users are able to claim tokens given a valid merkle proof.
- [Automated Market Maker (AMM)](./AMM/) is a decentralized exchange protocol that manages liquidity pools supplied by its users and determines prices algorithmically while exchanging assets.
- [Decentralized Autonomous Organization (DAO)](./DAO) is an organization where users get to vote on governance proposals using governance tokens.
- [English Auction](./auctions/english-auction/) is an auction where users bid up the price of an asset until the bidding period has ended or a reserve has been met.
- [Escrow](./escrow) is a third party that keeps an asset on behalf of multiple parties.
- [Fundraiser](./fundraiser/) is a program allowing users to pledge towards a goal.
- [Fractional-NFT](./fractional-NFT/) allows multiple parties to claim ownership of an NFT directly proportional to the number of tokens they hold.
- [Multi-Signature Wallet](./multisig-wallet) is a wallet that requires multiple signatures to execute a transaction.
- [Name-Registry](./name-registry/) allows users to perform transactions with human readable names instead of addresses.
- [Non-Fungible Token (NFT)](./NFT) is a token contract which provides unqiue collectibles, identified and differentiated by token IDs, where tokens contain metadata giving them distinctive characteristics.
- [Oracle](./oracle) is a smart contract that provides off-chain data to on-chain applications.
- [OTC Swap Predicate](./OTC-swap-predicate) is a predicate that can be used to propose and execute an atomic swap between two parties without requiring any on-chain state.

## Running a project

If you wish to run any of the projects then clone this repository and go through the general [installation](https://fuellabs.github.io/sway/) steps required to use our tools.

Any instructions related to running a specific project should be found within the README.md of that project.

> **Note**
> All projects currently use `forc 0.32.2`, `fuel-core 0.15.1`, and `rust 1.66.0`.

## Contributing

Check out the [book](https://fuellabs.github.io/sway-applications/book/index.html) for more info!
