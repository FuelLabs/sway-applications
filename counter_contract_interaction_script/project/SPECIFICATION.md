Table of Contents
- [Overview of the Contract](#Overview-of-Contract)
  - [Core Functionality](#core-functionality)
    - [`increment()`](#increment)
    - [`clear()`](#clear)
  - [State Checks](#state-checks)
    - [`count()`](#count)
- [Overview of the Script](#Overview-of-Script)
  - [Parameters](#Parameters)
    - [`counter_contract_id`](#counter_contract_id:-ContractId)
    - [`clear_count`](#clear_count:-bool)
  - [Execution flow](#Execution-flow)
- [Overview of the Rust integration test](#Overview-of-the-Rust-integration-test)

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

This is the ContractId of the CounterContract that we want to interact with

### `clear_count: bool`

This is a boolean value which determines if the script should call the `clear` method at the end of the script execution

## Execution flow

The script calls all 3 methods of the contract in a specific order, along with asserting the retrieved values from the contract in order to demonstrate that the contract is behaving as expected.

Note: "asserted" refers to the use of the `require` function, which asserts, or requires, that the input condition be true. If the input condition is false, the script stops executing and reverts.

Following is the order of execution:
- the `count` method is called and the returned value is stored in a variable `a`
- `a` is asserted to be equal to 0, as that is the default value of the `count` variable
- the `increment` method is called and the returned value is stored in the variable `a`
- `a` is now asserted to be equal to 1, as after incrementing 0 by 1, the result will be 1.
- the `increment` method is called again and the returned value is stored in the variable `a`
- `a` is now asserted to be equal to 2, as after incrementing 1 by 1, the result will be 2.
- If the value of the parameter `clear_count`, which was passed into the script main function, is true, the `clear` method is called.
- the `count` method is called and the returned value is stored in a variable `a`
- the `a` variable is returned, script execution is finished.

# Overview of the Rust integration test

The rust integration test deploys the CounterContract, and then executes the script. One of the tests is with the `clear_count` set to true, another test is with the `clear_count` set to false.

