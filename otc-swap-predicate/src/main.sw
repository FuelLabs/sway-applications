predicate;

use std::{
    address::Address,
    contract_id::ContractId,
    option::Option,
tx:: {
    b256_from_pointer_offset, tx_input_owner, tx_inputs_count, tx_output_amount, tx_output_pointer
}
};

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
    if (tx_inputs_count() == 2) {
        let owner = match tx_input_owner(0) {
            Option::Some(owner) => owner, _ => tx_input_owner(1).unwrap(), 
        };

        if (owner == receiver) {
            return true;
        };
    };

    // Otherwise, evaluate the terms of the order:
    // The output which pays the receiver must be in the first position (index = 0)

    let amount = tx_output_amount(0);

    // Get the token contract ID and receiver from the output
    let output_pointer = tx_output_pointer(0);

    // `Output::Coin` is serialized as :
    //    `type`     (8 bytes)
    //    `to`       (32 bytes)
    //    `amount`   (8 bytes)
    //    `asset_id` (32 bytes)
    // Offsets from the output pointer to each property are set accordingly:

    let to = ~Address::from(b256_from_pointer_offset(output_pointer, 8));
    let asset_id = ~ContractId::from(b256_from_pointer_offset(output_pointer, 48));

    // Evaluate the predicate
    (to == receiver) && (amount == ask_amount) && (asset_id == ask_token)
}
