# Overview

## Predicates in Fuel
Predicates are pure functions evaluating to either True or False. They are stateless, and can neither read nor write to any contract state. They can not emit logs.

In Fuel, coins can be sent to an address representing a particular predicate's bytecode (the bytecode root, calculated [here](https://github.com/FuelLabs/fuel-specs/blob/master/specs/protocol/identifiers.md#contract-id)).


These coin UTXOs then become spendable not on the provision of a valid signature of the owner, but rather if the supplied predicate both has a root matches the owner of the input being spent, and [evaluates](https://github.com/FuelLabs/fuel-specs/blob/master/specs/vm/main.md#predicate-verification) to True

Predicates may introspect the transaction spending their coins (inputs, outputs, script bytecode, etc.) and may take runtime arguments (the `predicateData`)which affect the evaluation of the predicate.

## Order / OTC swap Predicate

This predicate serves as an "order" that anyone can fill. The order "maker" transfers a coin to the predicate root which be unlocked by any transaction which has an output which meets the conditions of the order : the spending transaction must transfer `ask_amount` of `ask_token` to the `maker`. These constants are hard-coded in the predicate itself, so that the bytecode root commits to this specific set of conditions.

The order "taker" can then execute the order by spending the predicate. They are free to spend the predicate's coin in any way they wish, so long as the transaction has an output which satisfies the above conditions. They must provide an index to this output in the predicate data.

The order maker can "cancel" the order by spending the predicate's coins in a transaction containing a single coin input they have signed

Limitations:
- An order can not be partially filled - the taker must pay the entire ask amount. (They do not have to take the full amount offered in the predicate, but not do do so would be beneficial only to the order maker)
- There is no on-chain matching engine, so an order placed "offside" would not be matched with an existing order with a better price (on the contrary, it would be vulnerable to arbitrage)

As such, this mechanism is most useful for OTC trades and atomic swaps.

# Repository Structure


# Running the project


# Contributing

Check [CONTRIBUTING.md](../CONTRIBUTING.md) for more info!