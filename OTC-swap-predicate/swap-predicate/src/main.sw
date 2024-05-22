predicate;

use std::{
    inputs::{
        input_coin_owner,
        input_count,
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
    ASK_ASSET: AssetId = AssetId::from(0x0101010101010101010101010101010101010101010101010101010101010101),
    /// The receiver to whom the swapped asset will be sent.
    RECEIVER: Address = Address::from(0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db),
}

/// Validates conditions within the transaction to perform a swap
///
/// # Additional Information
///
/// The user can cancel their order by including an input coin from themselves.
///
/// # Returns
///
/// * [bool] - `true` if the spender is the receiver or if the terms of the order are met, `false` otherwise.
fn main() -> bool {
    // The spending transaction must have an output that sends `ask_amount` of `ask_asset` to `receiver`

    // Check if the transaction contains a single input coin from the receiver, to cancel their own order (in addition to this predicate)
    if input_count() == 2u8 {
        match (input_coin_owner(0), input_coin_owner(1)) {
            (Some(owner1), Some(owner2)) => {
                if owner1 == RECEIVER || owner2 == RECEIVER {
                    return true;
                }
            }
            _ => return false,
        }
    }

    // Otherwise, evaluate the terms of the order:
    // The output which pays the receiver must be the first output
    let output_index = 0;

    // Revert if output is not an Output::Coin
    match output_type(output_index) {
        Output::Coin => (),
        _ => return false,
    };

    // Since output is known to be a Coin, the following are always valid
    // let to = Address::from(output_asset_to(output_index).unwrap());
    // let asset_id = output_asset_id(output_index).unwrap();

    let to = match output_asset_to(output_index) {
        Some(address) => address,
        None => return false,
    };
    let to = Address::from(to);

    let asset_id = match output_asset_id(output_index) {
        Some(asset_id) => asset_id,
        None => return false,
    };

    let amount = output_amount(output_index);

    // Evaluate the predicate
    (to == RECEIVER) && (amount == ASK_AMOUNT) && (asset_id == ASK_ASSET)
}
