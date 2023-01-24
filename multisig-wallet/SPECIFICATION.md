Table of Contents

- [Overview](#overview)
- [Use Cases](#use-cases)
  - [Core Functionality](#core-functionality)
    - [`cancel_transaction()`](#cancel_transaction)
    - [`constructor()`](#constructor)
    - [`execute_transaction()`](#execute_transaction)
    - [`set_threshold()`](#set_threshold)
    - [`transfer()`](#transfer)
  - [State Checks](#state-checks)
    - [`balance()`](#balance)
    - [`nonce()`](#nonce)
    - [`threshold()`](#threshold)
  - [Utilities](#utilities)
    - [`transaction_hash()`](#transaction_hash)
    - [`update_hash()`](#update_hash)
  - [Sequence Diagram](#sequence-diagram)

# Overview

This document provides an overview of the application.

It outlines the use cases, i.e., desirable functionality, in addition to requirements for the smart contracts.

# Use Cases

This section contains general information about the functionality of the application and thus does not touch upon any technical aspects.

If you are interested in a functional overview then this is the section for you.

## Core Functionality

### `cancel_transaction()`

1. Cancels the next transaction by spending the current nonce. This is both a safety mechanism enabling a user to explicitly render a previously shared signature useless, as well as a way of conveniently skipping a transaction.
   1. If the caller is an owner, which requires the contract to have been initialised.

### `constructor()`

1. Sets the parameters for approving a transaction and sets the owners of the multisig.
   1. If the constructor hasn't already been called.
   2. Requires the config time constant `THRESHOLD`; the number of approvals required for a transaction to occur.
      1. If the `THRESHOLD` is not 0.
   3. Requires the `users`; the information about the owners of the multisig
      1. If none of the owners have the 0th address (0x00000...).
      2. If none of the owners are set to have an approval weighting (number of approvals per owner) of 0.
      3. If the sum of the owners' approval weightings is a value larger than the `threshold` parameter. This prevents the contract being setup when the owners can never submit enough approvals to allow a transaction.

### `execute_transaction()`

1. Execute a transaction, formed from the parameters.
   > **NOTE** This functionality is not yet fully implemented.
   1. If the constructor has been called.
   2. If signature recovery is successful.
   3. If the recovered addresses are in ascending order.
   4. If the number of approvals, from the owners whose addresses were recovered, meets the threshold.
   5. Requires `data`; the data field of the transaction to be executed.
   6. Requires `signatures`; The information for each of the signatures submitted to approve a specific transaction.
   7. Requires `to`; The recipient of the transaction to be executed.
   8. Requires `value`; The value sent in the transaction to be executed.

### `set_threshold()`

1. Changes the threshold required for execution of transactions.
2. Reverts when:
   1. The constructor has not been called.
   2. When the new threshold is greater than the total weight of the owners.
   3. Signature recovery failed.
   4. Recovered addresses are not in ascending order.
   5. The number of approvals does not meet the threshold

### `transfer()`

1. Transfers assets, via a transaction formed from the parameters.
   1. If the constructor has been called.
   2. If signature recovery is successful.
   3. If the recovered addresses are in ascending order.
   4. If the number of approvals, from the owners whose addresses were recovered, meets the threshold.
   5. Requires `asset_id`: the contract ID of the asset to be transferred.
   6. Requires `data`; the data field of the transaction.
   7. Requires `signatures`; The information for each of the signatures submitted to approve a specific transaction.
   8. Requires `to`; The recipient of the transaction.
   9. Requires `value`; The value sent in the transaction.
      1. If the contract owns enough of the asset to be transferred.

## State Checks

### `balance()`

1. Returns the contract's balance of the specified asset.
   1. Requires `asset_id`; The contract ID of the asset to check that balance of.

### `nonce()`

1. Returns the current nonce of the contract.

### `threshold()`

1. Returns the threshold for execution.

## Utilities

### `transaction_hash()`

1. Returns the hash of a transaction, comprised of the parameters. This is a utility for getting a transaction hash to sign over.
   1. Requires `data`; The data field of the transaction.
   2. Requires `nonce`; The nonce field of the transaction.
   3. Requires `to`; The recipient of the transaction.
   4. Requires `value`; The value sent in the transaction.

### `update_hash()`

1. Creates a hash which is used to make updates to the state of the contract
   1. Requires `data`; The data field of the transaction.
   2. Requires `nonce`; The nonce field of the transaction.
2. Exists for the purpose of UX i.e. updating the state of the contract does not require the user to enter a `to` or `value`

## Sequence Diagram

![Multisig wallet Sequence Diagram](.docs/multisig-wallet-sequence-diagram.png)
