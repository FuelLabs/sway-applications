predicate;

use std::{
    address::Address,
    assert::assert,
    constants::BASE_ASSET_ID,
    contract_id::ContractId,
    option::Option,
    result::Result,
    revert::revert,
    tx::{b256_from_pointer_offset, tx_input_owner, tx_output_amount, tx_output_pointer}
};

/// Order / OTC swap Predicate
///
/// Sending tokens to this predicate root creates an "order" that anyone can fill
/// This coin can be unlocked by any transaction which has an output which meets the conditions of the oder
/// The order maker can "cancel" the order by spending the predicate's coins in any transaction containing an input they have signed
///
/// Limitations:
///    - An order can not be partially filled - the taker must pay the entire ask amount
///    - There is no on-chain matching engine, so an order placed 'offside' would not be matched with an existing order with a better price (on the contrary, it would be vulnerable to arbitrage)
///
/// As such, this mechanism is most useful for OTC trades and atomic swaps.
///
/// # Arguments
///
/// - `input_index` - The index of the Coin input signed by the order maker, if the order is to be cancelled.
/// - `output_index` - The index of the Coin output which pays the order maker.
///
fn main(input_index: u8, output_index: u8) -> bool {
    // Order conditions: This must be hardcoded here.
    // The spending transaction must have an output that sends `ask_amount` of `ask_token` to `maker`
    let maker = ~Address::from(0x0303030303030303030303030303030303030303030303030303030303030303);
    let ask_amount = 42;
    let ask_token = BASE_ASSET_ID;

    // First, check if the transaction contains an input coin from the maker, to cancel their own order:
    let taker = tx_input_owner(input_index).unwrap();

    if (taker == maker) {
        true
    }

    else {
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
}
