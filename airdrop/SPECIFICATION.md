Table of Contents
- [Overview](#overview)
- [Use Cases](#use-cases)
  - [Actions that users are able to perform](#actions-that-users-are-able-to-perform)
    - [Create Airdrop](#create-airdrop)
    - [Claim Airdrop](#claim-airdrop)
  - [Information that is presented to users](#information-that-is-presented-to-users)
    - [Airdrops](#airdrops)
    - [Balances](#balances)
    - [Discovery of Airdrops](#discovery-of-airdrops)
- [Specification](#specification)
  - [Airdrop-Distributor Smart Contract](#airdrop-distributor-smart-contract)
    - [Starting an Airdrop](#starting-an-airdrop)
      - [Context](#context)
      - [Requirements](#requirements)
    - [Claiming an Airdrop](#claiming-an-airdrop)
      - [Context](#context-1)
      - [Requirements](#requirements-1)
    - [Helper functions for the user interface](#helper-functions-for-the-user-interface)
      - [End Block](#end-block)
        - [Context](#context-2)
        - [Requirements](#requirements-2)
      - [Merkle Root](#merkle-root)
        - [Context](#context-3)
        - [Requirements](#requirements-3)
  - [Simple-Token Smart Contract](#simple-token-smart-contract)
    - [Creating A Token](#creating-a-token)
      - [Context](#context-4)
      - [Requirements](#requirements-4)
    - [Minting Tokens](#minting-tokens)
      - [Context](#context-5)
      - [Requirements](#requirements-5)
  - [User interface](#user-interface)

# Overview

This document provides an overview of the application.

It outlines the use cases, i.e. desirable functionality, in addition to requirements for the smart contract and the user interface.

This application inherits specification from the [Binary Merkle Proof Verification Library specification](https://github.com/FuelLabs/sway-libs/blob/master/sway_libs/src/merkle_proof/SPECIFICATION.md).

# Use Cases

This section contains general information about the functionality of the application and thus does not touch upon any technical aspects.

If you are interested in a functional overview then this is the section for you.

## Actions that users are able to perform

This sub-section details what a user is able to do e.g. click a button and "x, y, z" happens.

### Create Airdrop

A user should be able to create an airdrop which consists of 

1. A token which will be distributed
2. A Merkle root constructed from a Merkle Tree where each leaf is the resulting `sha-256` hash of a `(user address, amount)` tuple
3. A length of time in which a claim is valid

### Claim Airdrop

A user should be able to claim their airdrop

1. Before the claim period is over
2. If they has not yet claimed
3. If the amount they are claiming is verifiable
4. The proof provided is valid

## Information that is presented to users

This sub-section details the information that a user should have access to / what the application provides to them e.g. a history of their previous actions.

### Airdrops

A user should be able to see a history of the airdrop that they have created

1. This should be categorized into currently active and expired airdrops
2. The user should see
    1. When an airdrop ends / time until expires
    2. Which airdrops have been claimed
    3. Which token the airdrop distributes

### Balances

Users should be able to see the balance of their address

1. This should show a list of tokens which they have claimed
2. When claimed the amount should increase

### Discovery of Airdrops

Users should be able to see a list of airdrops which they may attempt to make a claim from. 

# Specification

## Airdrop-Distributor Smart Contract

### Starting an Airdrop

#### Context

An airdrop must specify the token which will mint tokens when a claim is made. The airdrop contract also requires a merkle root to be posted which will be verified against as well as a claim period in which the airdrop is active.

#### Requirements

- The following parameters are required
    - Time the airdrop is active
        - `type:` u64
        - `restrictions:`
            - Cannot be zero
    - Computed Merkle root
        - `type:` b256
        - `description:` The resulting computed root hash from a Merkle Tree containing user address and claim amounts
    - Distributing token
        - `type`: ContractId
        - `description:` The token which will be minted upon claims
- The airdrop must also track
    - Claims each user has made with an amount
- Anyone can call the function to create an airdrop
- When a new airdrop is create a log should be emitted containing
    - The block at which the airdrop expires
    - The Merkle root
    - The token which will be minted upon claims

### Claiming an Airdrop

#### Context

A claim to an airdrop can be made given the appropriate parameters and the validity of their claim. Once the claim is made and proven to be true, the specified number of tokens will be minted to the user that claimed.

#### Requirements

- The following parameters are required
    - Amount of tokens
        - `type:` u64
        - `description:` The number of tokens which are to be minted upon the claim
    - Proof leaf index
        - `type:` u64
        - `description:` The index in the Merkle Tree which the corresponds to the key at which the user's claim is set
        - `restrictions:`
            - Cannot be greater than the number of leaves in the Merkle Tree
    - Number of Merkle leaves
        - `type:` u64
        - `description:` The number of Merkle leaves in the Merkle Tree
    - Merkle Proof
        - `type:` Vec<b256>
        - `description:` The proof which is used to verify the validity of the claim upon the tokens
    - User with a claim
        - `type:` Identity
        - `description:` The user which has a claim for the airdrop
- The airdrop claim must verify the validity of the leaf created by the `sha-256` hash of the `(User with a claim, Amount of tokens)` tuple with the provided proof against the Merkle root
- The airdrop claim must ensure a user cannot claim after the airdrop has expired
- The airdrop claim must ensure a user cannot claim twice
- The airdrop must use the [Binary Merkle Proof Verification Library](https://github.com/FuelLabs/sway-libs/tree/master/sway_libs/src/merkle_proof)
- Anyone can call the function to make a claim
- When a new airdrop is create a log should be emitted containing
    - The user which claimed
    - The amount of tokens claimed

### Helper functions for the user interface

#### End Block

##### Context

The user interface will need to be able to retrieve information reguarding the experation of the airdrop.

##### Requirements

- The following values should be returned
    - Experation block
        - `type:` u64
        - `description:` The block at which the airdrop is after no longer valid. This should be compared to the current block to determine if the airdrop has ended
- Anyone can call the function

#### Merkle Root

##### Context

The user interface may need to be able to retrieve information such as the stored Merkle root for the airdrop

##### Requirements

- The following values should be returned
    - Merkle root
        - `type:` b256
        - `description:` The stored Merkle root used to ensure validity of claims
- Anyone can call the function

## Simple-Token Smart Contract

### Creating a Token

#### Context

The user will need to create a token that is to be minted when distributed by the airdrop.

#### Requirements

- The following parameters are required
    - User with permission to mint
        - `type:` Identity
        - `description:` The token will only allow this permissioned user to mint new tokens.
    - Token Supply
        - `type:` u64
        - `description:` The total number of tokens that may ever be minted
        - `restrictions:`
            - Cannot be zero
- The function must ensure that the constuctor has not yet been called by ensuring the token supply is zero
- Anyone can call the function

### Minting Tokens

#### Context 

A specified number of tokens shall be minted to an address.

#### Requirements

- The following parameters are required
    - Amount of tokens
        - `type:` u64
        - `description:` The number of tokens which are to be minted
        - `restrictions:`
            - The amount of previously minted tokens plus the new tokens to be minted shall not exceed the total supply of tokens
    - Receiving User
        - `type:` Identity
        - `description:` The user which will receive the newly minted tokens
- The function shall ensure that the user calling the function is permissioned to mint new tokens
- Anyone can call the function

## User Interface
