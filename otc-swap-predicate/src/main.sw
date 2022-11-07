predicate;

use std::{
    inputs::{
        input_count,
        input_owner,
    },
    outputs::{
        Output,
        output_amount,
        output_pointer,
        output_type,
    },
};

// TO DO : Remove once __gtf getters implemented in std-lib
const GTF_OUTPUT_COIN_TO = 0x202;
const GTF_OUTPUT_COIN_ASSET_ID = 0x204;

/// Order / OTC swap Predicate
fn main() -> bool {
    // Order conditions: These are set in Forc.toml
    // The spending transaction must have an output that sends `ask_amount` of `ask_token` to `receiver`
    // Conversion to ContractId and Address types will be unnecessary once
    // https://github.com/FuelLabs/sway/issues/2647 is fixed
    let ask_token: ContractId = ContractId {
        value: ask_token_config,
    };
    let receiver = ~Address::from(receiver_config);

    // Check if the transaction contains a single input coin from the receiver, to cancel their own order
    // Note that the predicate is necessarily one of the inputs, so the other must be the coin input.
    if (input_count() == 2u8) {
        let owner = match input_owner(0) {
            Option::Some(owner) => owner,
            _ => input_owner(1).unwrap(),
        };

        if (owner == receiver) {
            return true;
        };
    };

    // Otherwise, evaluate the terms of the order:
    // TO DO : Have the order taker provide the index of the output via predicateData
    // The output which pays the receiver must be in the first position (output_index = 0)
    let output_index = 0;

    // Revert if output is not an Output::Coin
    match output_type(output_index) {
        Output::Coin => (),
        _ => revert(0),
    };

    // Since output is known to be a Coin, the following are always valid
    let to = ~Address::from(__gtf::<b256>(output_index, GTF_OUTPUT_COIN_TO));
    let asset_id = ~ContractId::from(__gtf::<b256>(output_index, GTF_OUTPUT_COIN_ASSET_ID));

    let amount = output_amount(output_index);

    // Evaluate the predicate
    (to == receiver) && (amount == ask_amount) && (asset_id == ask_token)
}
