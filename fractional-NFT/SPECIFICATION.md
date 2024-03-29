Table of Contents
- [Overview](#overview)
- [Use Cases](#use-cases)
- [Fractional NFT Contract](#fractional-nft-contract)
  - [SRC-6](#src-6)
    - [`deposit()`](#deposit)
    - [`withdraw()`](#withdraw)
    - [`managed_assets()`](#managed_assets)
    - [`max_depositable()`](#max_depositable)
    - [`max_withdrawable()`](#max_withdrawable)
  - [SRC-20 Functionality](#src-20-functionality)
    - [`total_assets()`](#total_assets)
    - [`total_supply()`](#total_supply)
    - [`name()`](#name)
    - [`symbol()`](#symbol)
    - [`decimals()`](#decimals)
    - [`set_name()`](#set_name)
    - [`set_symbol()`](#set_symbol)

# Overview

This document provides an overview of the application.

It outlines the use cases, i.e. desirable functionality, in addition to requirements for the smart contract and the user interface.

# Use Cases

This section contains general information about the functionality of the application and thus does not touch upon any technical aspects.

If you are interested in a functional overview then this is the section for you.

# Fractional NFT Contract

## SRC-6

### `deposit()`

This function will mint new shares when an NFT is deposited.

### `withdraw()`

This function will burn all F-NFT shares and release the locked NFT.

### `managed_assets()`

This function will return the number of assets managed under a specific vault. 

> **NOTE:** In this application this is always a maximum of 1 as NFTs have a supply of 1.

### `max_depositable()`

This function will return the maximum number of assets that can be deposited for a vault.

> **NOTE:** In this application this is always a maximum of 1 as NFTs have a supply of 1.

### `max_withdrawable()`

This function will return the maximum number of assets that can be withdrawn from a vault.

> **NOTE:** In this application this is always a maximum of 1 as NFTs have a supply of 1.


## SRC-20 Functionality

### `total_assets()`

This function will return the total number of individual assets for a contract.

### `total_supply()`

This function will return the total supply of coins for an asset.

### `name()`

This function will return the name of an asset, such as “Ether”.

### `symbol()`

This function will return the symbol of an asset, such as “ETH”.

### `decimals()`

This function will return the number of decimals an asset uses.

### `set_name()`

This function will unconditionally set the name of an asset.

### `set_symbol()`

This function will unconditionally set the symbol of an asset.
