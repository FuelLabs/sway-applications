predicate;

use std::{
    address::Address,
    constants::BASE_ASSET_ID,
    contract_id::ContractId,
    option::Option,
tx:: {
    b256_from_pointer_offset, tx_input_owner, tx_inputs_count, tx_output_amount, tx_output_pointer
}
};

/// Order / OTC swap Predicate
/// # Arguments
///
/// - `output_index` - The index of the Coin output which pays the order maker.
///
fn main(output_index: u8) -> bool {
    // Order conditions: This must be hardcoded here.
    // The spending transaction must have an output that sends `ask_amount` of `ask_token` to `maker`
    let maker = ~Address::from(0x0303030303030303030303030303030303030303030303030303030303030303);
    let ask_amount = 42;
    let ask_token = BASE_ASSET_ID;

    // Check if the transaction contains a single input coin from the maker, to cancel their own order
    // Note that the predicate is necessarily one of the inputs, so the other must be the coin input.
    if (tx_inputs_count() == 2) {
        let owner = match tx_input_owner(0) {
            Option::Some(owner) => owner, _ => tx_input_owner(1).unwrap(), 
        };

        if (owner == maker) {
            return true;
        };
    };

    // Otherwise, evaluate the terms of the order:

    let amount = tx_output_amount(output_index);

    // Get the token contract ID and receiver from the output at the given index
    let output_pointer = tx_output_pointer(output_index);

    // `Output::Coin` is serialized as :
    //    `type`     (8 bytes)
    //    `to`       (32 bytes)
    //    `amount`   (8 bytes)
    //    `asset_id` (32 bytes)
    // Offsets from the output pointer to each property are set accordingly:

    let to = ~Address::from(b256_from_pointer_offset(output_pointer, 8));
    let asset_id = ~ContractId::from(b256_from_pointer_offset(output_pointer, 48));

    // Evaluate the predicate
    (to == maker) && (amount == ask_amount) && (asset_id == ask_token)
}
