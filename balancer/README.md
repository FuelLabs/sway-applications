# Balancer V2 


This repository contains the Balancer Protocol V2 core smart contracts written in sway language for fuelVM, including the weighted Pools, along with their tests, configuration, and deployment information.

## Structure

This is a Smart Contract Development repository for Balancer Protocol For FuelVM, with the packages meant to be published in this directory. Newly developed packages may not be published yet.

Active development occurs in this repository, which means some contracts in it might not be production-ready. Proceed with caution.

### Packages

- [`pool-utils`](./pool-utils/contracts/): Sway utilities used to develop Pool contracts.
- [`Wieghted-pool`](./weighted-pool/contracts/): Weighted Pool Smart contracts
- [`Sway-utils`](./sway-utils/contracts/): miscellaneous Sway helpers and utilities used in many different contracts.
- [`Vault`](./vault/contracts/): asset manager for the balancer protocol. connects all the pools
