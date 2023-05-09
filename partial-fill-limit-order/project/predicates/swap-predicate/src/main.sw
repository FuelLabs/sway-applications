predicate;


use fixed_point::ufp64::UFP64;

use std::{
    inputs::{
        Input,
        input_amount,
        input_count,
        input_owner,
        input_type,
    },
    outputs::{
        Output,
        output_amount,
        output_count,
        output_type,
    },
};

// TODO : Remove once __gtf getters implemented in std-lib
const GTF_OUTPUT_COIN_TO = 0x202;
const GTF_OUTPUT_COIN_ASSET_ID = 0x204;

configurable {
    ASK_TOKEN: ContractId = ContractId::from(0x0101010101010101010101010101010101010101010101010101010101010101),
    RECEIVER: Address = Address::from(0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db),
    ASK_PRICE: UFP64 = UFP64 { value: 42949672960 }, // Equal to UFP64::from_uint(10)
}

/// Order / OTC swap Predicate
fn main() -> bool {
    // Order conditions: These are set in the configurable block above
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
    let to = Address::from(__gtf::<b256>(output_index, GTF_OUTPUT_COIN_TO));
    let asset_id = ContractId::from(__gtf::<b256>(output_index, GTF_OUTPUT_COIN_ASSET_ID));

    let amount_to_reciever = output_amount(output_index);
    let amount_to_reciever = UFP64::from_uint(amount_to_reciever);

    // The input which pays the bidder must be the first input
    let input_index = 0;

    // Revert if input is not an Input::Coin
    match input_type(input_index) {
        Input::Coin => (),
        _ => revert(0),
    };

    // Since input is known to be a Coin, the following are always valid
    let input_amount = input_amount(input_index).unwrap();
    let input_amount = UFP64::from_uint(input_amount);

    // Evaluate the predicate
    // (to == RECEIVER) && (amount == ASK_AMOUNT) && (asset_id == ASK_TOKEN)

    if (to == RECEIVER) && (asset_id == ASK_TOKEN) {
        let output_count = output_count();
        if output_count == 1_u8 {
            if input_amount * ASK_PRICE == amount_to_reciever {
                true
            }
        } else if output_count == 2_u8 {
            let this_predicates_address = todo!();

            let output_index = 1_u8;

            // Revert if output is not an Output::Coin
            match output_type(output_index) {
                Output::Coin => (),
                _ => revert(0),
            };

            // Since output is known to be a Coin, the following are always valid
            let to2 = Address::from(__gtf::<b256>(output_index, GTF_OUTPUT_COIN_TO));
            let asset_id2 = ContractId::from(__gtf::<b256>(output_index, GTF_OUTPUT_COIN_ASSET_ID));

            let amount_back_to_predicate = output_amount(output_index);

            if (to2 == this_predicates_address) && (asset_id2 == ASK_TOKEN) {
                if input_amount * ASK_PRICE == amount_to_reciever + amount_back_to_predicate {
                    true
                }
            }
        }
    }

    false
}
