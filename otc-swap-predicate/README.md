<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset=".docs/otc-swap-predicate-logo-dark-theme.png">
        <img alt="otc swap predicate logo" width="400px" src=".docs/otc-swap-predicate-logo-light-theme.png">
    </picture>
</p>

# Overview

## Predicates in Fuel
Predicates are pure functions evaluating to either `True` or `False`. They are stateless, and can neither read nor write to any contract state. They can not emit logs.

In Fuel, coins can be sent to an address uniquely representing a particular predicate's bytecode (the bytecode root, calculated [here](https://github.com/FuelLabs/fuel-specs/blob/master/specs/protocol/identifiers.md#contract-id)).


These coin UTXOs then become spendable not on the provision of a valid signature, but rather if the supplied predicate both has a root that matches their owner, and [evaluates](https://github.com/FuelLabs/fuel-specs/blob/master/specs/vm/main.md#predicate-verification) to `True`. If a predicate reverts, or tries to access impure VM opcodes, the evaluation is automatically `False`.

Predicates may introspect the transaction spending their coins (inputs, outputs, script bytecode, etc.) and may take runtime arguments (the `predicateData`), either or both of which may affect the evaluation of the predicate.

## Order / OTC swap Predicate

This predicate serves as an "order" that anyone can fill. The order maker transfers a coin to the predicate root which can be unlocked by any transaction which has an output that satisfies the conditions of the order : the spending transaction must transfer `ask_amount` of `ask_token` to the `receiver`. These constants are hard-coded in the predicate itself, so that the bytecode root commits to this specific set of conditions.

The order "taker" can then execute the order by spending the predicate. They are free to spend the predicate's coin in any way they wish, so long as the transaction has an output which satisfies the above conditions. This output must be the first output in the output set (i.e. its index is 0)

The order maker can "cancel" the order by spending the predicate's coins in a transaction containing a single coin input signed by the `receiver`. There are therefore two inputs: the signed coin and the predicate coin.

Limitations:
- An order can not be partially filled - the taker must pay the entire ask amount.
- There is no on-chain matching engine, so an order placed "offside" would not be matched with an existing order with a better price (on the contrary, it would be vulnerable to arbitrage).

As such, this mechanism is most useful for OTC trades and atomic swaps.

# Project Structure
The project consists of a predicate written in Sway (`/src/main.sw`) and tests using fuels-rs (`/tests/`)

# Running the project
In order to run the tests make sure that you are in the root of this project.

Build the predicate:  
`forc build`

Run the tests:  
`cargo test`
