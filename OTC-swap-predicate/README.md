<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/otc-swap-predicate-logo-dark-theme.png">
        <img alt="otc swap predicate logo" width="400px" src=".docs/otc-swap-predicate-logo-light-theme.png">
    </picture>
</p>

<p align="center">
    <a href="https://crates.io/crates/forc/0.46.0" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.46.0-orange" />
    </a>
    <a href="https://crates.io/crates/fuel-core/0.20.5" alt="fuel-core">
        <img src="https://img.shields.io/badge/fuel--core-v0.20.5-yellow" />
    </a>
    <a href="https://crates.io/crates/fuels/0.49.0" alt="forc">
        <img src="https://img.shields.io/badge/fuels-v0.49.0-blue" />
    </a>
</p>

## Overview

## Predicates in Fuel

Predicates are pure functions evaluating to either `True` or `False`. They are stateless, and can neither read nor write to any contract state. They can not emit logs.

In Fuel, coins can be sent to an address uniquely representing a particular predicate's bytecode (the bytecode root, calculated [here](https://github.com/FuelLabs/fuel-specs/blob/master/src/protocol/id/contract.md)).

These coin UTXOs then become spendable not on the provision of a valid signature, but rather if the supplied predicate both has a root that matches their owner, and [evaluates](https://github.com/FuelLabs/fuel-specs/blob/master/src/vm/index.md#predicate-verification) to `True`. If a predicate reverts, or tries to access impure VM opcodes, the evaluation is automatically `False`.

Predicates may introspect the transaction spending their coins (inputs, outputs, script bytecode, etc.) and may take runtime arguments (the `predicateData`), either or both of which may affect the evaluation of the predicate.

## Order / OTC swap Predicate

This predicate serves as an "order" that anyone can fill. The order maker transfers a coin to the predicate root which can be unlocked by any transaction which has an output that satisfies the conditions of the order : the spending transaction must transfer `ask_amount` of `ask_token` to the `receiver`. These constants are hard-coded in the predicate itself, so that the bytecode root commits to this specific set of conditions.

The order "taker" can then execute the order by spending the predicate. They are free to spend the predicate's coin in any way they wish, so long as the transaction has an output which satisfies the above conditions. This output must be the first output in the output set (i.e. its index is 0)

The order maker can "cancel" the order by spending the predicate's coins in a transaction containing a single coin input signed by the `receiver`. There are therefore two inputs: the signed coin and the predicate coin.

Limitations:

- An order can not be partially filled - the taker must pay the entire ask amount.
- There is no on-chain matching engine, so an order placed "offside" would not be matched with an existing order with a better price (on the contrary, it would be vulnerable to arbitrage).

As such, this mechanism is most useful for OTC trades and atomic swaps.

## Project structure

The project consists of a predicate.

```sh
OTC-swap-predicate
├── project
│   ├── predicates
│   │   └── swap-predicate
│   │       ├── src/main.sw
│   │       └── tests/harness.rs
│   ├── README.md
│   └── SPECIFICATION.md
├── ui
│   ├── README.md
│   └── SPECIFICATION.md
└── README.md
```

## Running the project

### User interface

TODO: The user interface does not currently exist therefore its [README.md](ui/README.md) and [SPECIFICATION.md](ui/SPECIFICATION.md) are empty.

### Project

In order to run the subsequent commands change into the following directory `/path/to/OTC-swap-predicate/project/<here>`.

#### Program compilation

```bash
forc build --locked
```

#### Running the tests

Before running the tests the programs must be compiled with the command above.

```bash
cargo test --locked
```
