Table of Contents
- [Overview](#overview)
- [Use Cases](#use-cases)
  - [Actions that users are able to perform](#actions-that-users-are-able-to-perform)
  - [Information that is presented to users](#information-that-is-presented-to-users)

# Overview

This document provides an overview of the application.

It outlines the use cases, i.e. desirable functionality, in addition to requirements for the smart contract and the user interface.

# Use Cases

This section contains general information about the functionality of the application and thus does not touch upon any technical aspects.

If you are interested in a functional overview then this is the section for you.

## Actions that users are able to perform

This sub-section details what a user is able to do e.g. click a button and "x, y, z" happens.

### `stake()`

A user can deposit the staking-token in order to earn the reward-token

1. The asset sent must be the staking-token
2. The amount sent must be greater than 0

### `withdraw()`

The user can withdraw their staking-tokens.

1. The total amount cannot exceed the total deposits
2. The amount must be more than 0

### `get_reward()`

The user can withdraw their earned rewards in the form of reward-tokens

### `exit()`

The user can withdraw and get their rewards at the same time.

1. Withdraws the total deposited amount

## Information that is presented to users

This sub-section details the information that a user should have access to / what the application provides to them e.g. a history of their previous actions.

### `balance_of()`

Returns the staked amount of the given Identity

### `earned()`

Returns the earned reward-tokens from staking of a given Identity

### `get_reward_for_duration()`

Returns the reward rate multiplied by the rewards duration. IE the reward rate if staked for the entire duration

### `last_update_time()`

Returns the last time any interaction with the contract occured

### `period_finish()`

Returns the timestamp when the staking rewards will end

### `staking_token()`

Returns the asset id which can be staked in the contract

### `rewards_token()`

Returns the asset id which the contract rewards for staking

### `total_supply()`

Returns the amount of tokens staked
