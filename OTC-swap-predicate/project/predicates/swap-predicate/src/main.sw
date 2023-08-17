predicate;

use std::{
    inputs::{
        input_count,
        input_owner,
    },
    outputs::{
        Output,
        output_amount,
        output_asset_id,
        output_asset_to,
        output_pointer,
        output_type,
    },
};

configurable {
    /// The amount of asset required to unlock the predicate.
    ASK_AMOUNT: u64 = 42,
    /// The asset to be paid.
    ASK_TOKEN: ContractId = ContractId::from(0x0101010101010101010101010101010101010101010101010101010101010101),
    /// The receiver of the ask.
    RECEIVER: Address = Address::from(0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db),
}

/// Order / OTC swap Predicate. Like a limit order.
///
/// # Additional Information
///
/// The user can cancel their own order by spending the output coin to themselves.
///
/// # Returns
///
/// * [bool] - `true` if the spender is the receiver or if the terms of the order are met. `false` otherwise.
fn main() -> bool {
    // The spending transaction must have an output that sends `ask_amount` of `ask_token` to `receiver`

    // Check if the transaction contains a single input coin from the receiver, to cancel their own order (in addition to this predicate)
    if input_count() == 2u8 {
        if input_owner(0).unwrap() == RECEIVER
            || input_owner(1).unwrap() == RECEIVER
        {
            return true;
        };
    };

    // Otherwise, evaluate the terms of the order:
    // The output which pays the receiver must be the first output
    let output_index = 0;

    // Revert if output is not an Output::Coin
    match output_type(output_index) {
        Output::Coin => (),
        _ => revert(0),
    };

    // Since output is known to be a Coin, the following are always valid
    let to = Address::from(output_asset_to(output_index).unwrap());
    let asset_id = output_asset_id(output_index).unwrap();

    let amount = output_amount(output_index);

    // Evaluate the predicate
    (to == RECEIVER) && (amount == ASK_AMOUNT) && (asset_id == ASK_TOKEN)
}
