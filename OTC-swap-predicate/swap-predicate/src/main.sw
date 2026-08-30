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
        output_type,
    },
};

configurable {
    /// The amount of asset required to unlock the predicate.
    ASK_AMOUNT: u64 = 42,
    /// The asset to be paid.
    ASK_ASSET: AssetId = AssetId::from(0x0101010101010101010101010101010101010101010101010101010101010101),
    /// The receiver address to whom the swapped asset will be sent.
    RECEIVER: Address = Address::from(0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db),
}

/// Validates conditions within the transaction to perform a swap.
///
/// # Additional Information
///
/// The user can cancel their order by including an input coin from themselves.
///
/// # Returns
///
/// * [bool] - `true` if the spender is the receiver or if the terms of the order are met, `false` otherwise.
fn main() -> bool {
    // The transaction must have an output that sends `ASK_AMOUNT` of `ASK_ASSET` to `RECEIVER`.

    // Check if the transaction contains exactly two input coins and if one of them belongs to the receiver
    if input_count() == 2u8 {
        match (input_coin_owner(0), input_coin_owner(1)) {
            (Some(owner1), Some(owner2)) => {
                // If either of the input coins belongs to the receiver, the transaction is valid.
                if owner1 == RECEIVER || owner2 == RECEIVER {
                    return true;
                }
            }
            _ => return false,
        }
    }

    // Otherwise, evaluate the terms of the order:
    // The output that pays the receiver must be the first output.
    let output_index = 0;

    // Revert if the output is not of type Output::Coin
    match output_type(output_index) {
        Output::Coin => (),
        _ => return false,
    };

    // Since the output type is known to be a Coin, we can safely retrieve the following details
    let to = match output_asset_to(output_index) {
        Some(address) => address,
        None => return false,
    };

    let asset_id = match output_asset_id(output_index) {
        Some(asset_id) => asset_id,
        None => return false,
    };

    let amount = output_amount(output_index);

    // Evaluate the predicate based on the provided conditions
    (to == RECEIVER) && (amount == ASK_AMOUNT) && (asset_id == ASK_ASSET)
}
