<p align="center">
    <img src="./.docs/logo.png" height="120">
</p>

<p align="center">
    <a href="https://github.com/FuelLabs/sway-applications/actions/workflows/ci.yml" alt="CI">
        <img src="https://github.com/FuelLabs/sway-applications/actions/workflows/ci.yml/badge.svg" />
    </a>
    <a href="https://crates.io/crates/forc/0.14.5" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.14.5-orange" />
    </a>
    <a href="https://discord.gg/xfpK4Pe">
        <img src="https://img.shields.io/discord/732892373507375164?color=blue&logo=discord&logoColor=ffffff&labelColor=6A7EC2&label=Discord" />
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

- [Escrow](./escrow) is a third party that keeps an asset on behalf of multiple parties
- [Multi-Signature Wallet](./multisig-wallet) is a wallet that requires multiple signatures to execute a transaction

## Running a project

If you wish to run any of the projects then clone this repository and go through the general [installation](https://fuellabs.github.io/sway/latest/introduction/installation.html) steps required to use our tools.

Any instructions related to running a specific project should be found within the README.md of that project.

## Contributing

Check [CONTRIBUTING.md](./CONTRIBUTING.md) for more info!
