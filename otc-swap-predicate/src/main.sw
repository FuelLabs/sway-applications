predicate;

use std::{
    address::Address,
    contract_id::ContractId,
    inputs::{
        input_count,
        input_owner,
    },
    option::Option,
    outputs::{
        Output,
        output_amount,
        output_pointer,
        output_type,
    },
    revert::revert,
};

const GTF_OUTPUT_COIN_TO = 0x202;
const GTF_OUTPUT_COIN_ASSET_ID = 0x204;

/// Order / OTC swap Predicate
fn main() -> bool {
    // Order conditions: This must be hardcoded here.
    // The spending transaction must have an output that sends `ask_amount` of `ask_token` to `receiver`
    let ask_amount = 42;
    let ask_token: ContractId = ContractId {
        value: 0x0101010101010101010101010101010101010101010101010101010101010101,
    };
    let receiver = ~Address::from(0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db);

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
    // The output which pays the receiver must be in the first position (output_index = 0)
    let output_index = 0;

    // TO DO : Replace the following with std-lib functions when available
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
