Table of Contents
- [Overview](#overview)
- [Use Cases](#use-cases)
  - [Core Functionality](#core-functionality)
    - [`register()`](#register)
    - [`extend()`](#extend)
    - [`set_asset()`](#set_asset)
    - [`set_resolver()`](#set_resolver)
    - [`transfer_name_ownership()`](#transfer_name_ownership)
  - [State Checks](#state-checks)
    - [`expiry()`](#expiry)
    - [`resolver()`](#resolver)
    - [`name_owner()`](#name_owner)
    - [`rate()`](#rate)

# Overview

This document provides an overview of the application.

It outlines the use cases, i.e. desirable functionality, in addition to requirements for the smart contract and the user interface.

# Use Cases

This section contains general information about the functionality of the application and thus does not touch upon any technical aspects.

If you are interested in a functional overview then this is the section for you.

## Core Functionality

This sub-section details what a user is able to do e.g. click a button and "x, y, z" happens.

### `register()`

A user can register a name for themselves

1. If the name is available for registration (ie unregistered or expired)
2. If the payment is in the correct asset
3. If the payment is sufficient for the duration

### `extend()`

Any user can extend the registration duration of a given name

1. If the given name is already registered
2. If the payment is in the correct asset
3. If the payment is sufficient for the duration

### `set_asset()`

Allows an owner to add, remove and alter the cost of an asset that the registry accepts

### `set_resolver()`

Allows the owner to change the resolving identity

### `transfer_name_ownership()`

Allows the owner to transfer ownership of the registered name

## State Checks

This sub-section details the information that a user should have access to / what the application provides to them e.g. a history of their previous actions.

### `expiry()`

Returns the expiry timestamp of a given name

1. If the name has been registered

### `resolver()`

Returns the identity to which the given name resolves to

1. If the name has been registered

### `name_owner()`

Returns the owner of the given name

1. If the name has been registered

### `rate()`

Returns the rate of cost for an asset
