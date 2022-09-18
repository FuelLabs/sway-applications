<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/fundraiser-logo-dark-theme.png">
        <img alt="SwayApps Fundraiser Logo" width="400px" src=".docs/fundraiser-logo-light-theme.png">
    </picture>
</p>

## Overview

This staking-rewards contract enables anyone to reward owners of a Token A with Token B, by locking up their tokens in the contract for a set amount of time. The user can withdraw at any time to get a pro-rata share of the rewards depending on how long they staked for

More information can be found in the [specification](./SPECIFICATION.md).

## Project Structure

The project consists of a smart contract and a user interface which the user can interact with.

<!--Only show most important files e.g. script to run, build etc.-->

```
staking-rewards/
├── contract/
|    └── src/staking_rewards.sw
|    └── tests/harness.rs
├── frontend/
|    └── Directories & files
├── README.md
└── SPECIFICATION.md
```

## Running the project

### User Interface

TODO: UI does not currently exist

### Tests

In order to run the tests make sure that you are in the root of this project i.e. `/path/to/staking-rewards/<you are here>`

Run the tests

   ```bash
   forc test
   ```

## Specification

The specification contains a non-technical overview of the contract indicating the flow of information from the start to the end of the staking-rewards contract.

Check [SPECIFICATION.md](./SPECIFICATION.md) for more info!

## Contributing

Check [CONTRIBUTING.md](../CONTRIBUTING.md) for more info!
