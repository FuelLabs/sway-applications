<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/nft-logo_white.png">
        <img alt="light theme" src=".docs/nft-logo_black.png">
    </picture>
</p>

## Overview

A non-fungible token (NFT) is a unique token that has an identifier which distinguishes itself from other tokens within the same token contract. At its core, there is nothing inherently special or unique with this implementation of an NFT besides the token ID. While it is commonly associated with artwork / collectibles, there are many greater utilities beyond that which have yet to be written for the Fuel Network.

### Current state of the application

- The smart contract is under development since the SDK is not feature rich enough to support the required tests
- The user interface does not exist and will not exist
  - The NFT in and of itself cannot be an application because it's a component that fits into other applications, such as auctions, therefore until the auction contracts are done the NFT cannot be considered an application
- The NFT is more of a library than an application therefore it will be moved into [Sway-Libs](https://github.com/FuelLabs/sway-libs) in the future

## Running the project

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/NFT/<you are here>`

Run the tests

```bash
forc test
```
