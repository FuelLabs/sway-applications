Table of Contents
- [Overview of the Contract](#Overview-of-Contract)
  - [Core Functionality](#core-functionality)
    - [`increment()`](#increment)
    - [`clear()`](#clear)
  - [State Checks](#state-checks)
    - [`count()`](#count)
- [Overview of the Script](#Overview-of-the-Script)
  - [Parameters](#Parameters)
    - [`counter_contract_id`](#counter_contract_id:-ContractId)
    - [`clear_count`](#clear_count:-bool)

# Overview of the Contract

The contract has an internal variable `count` that can be retrieved or manipulated by the user based on the methods called.

## Core Functionality

This sub-section details what a user is able to do e.g. click a button and "x, y, z" happens.

### `increment()`

Increases the internal count variable by 1.

### `clear()`

Sets the internal count variable to 0.

## State Checks

This sub-section details the information that a user should have access to / what the application provides to them e.g. a history of their previous actions.

### `count()`

Returns the current value of the internal `count` variable.

# Overview of the Script

The script demonstrates how to interact with a simple contract, with conditional execution and returning of values.

## Parameters

The script contains two input parameters

### `counter_contract_id: ContractId`

This is the ContractId of the counter contract that we want to interact with

### `clear_count: bool`

This is a boolean value which determines if the script should call the `clear` method at the end of the script execution
