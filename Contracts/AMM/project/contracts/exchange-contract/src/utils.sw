library;

use core::primitives::*;
use libraries::data_structures::{Asset, AssetPair};
use ::errors::{InitError, InputError};
use std::u128::U128;

fn calculate_amount_with_fee(amount: u64, liquidity_miner_fee: u64) -> u64 {
    let fee = (amount / liquidity_miner_fee);
    amount - fee
}

/// Returns the maximum required amount of the input asset to get exactly `output_amount` of the output asset
pub fn maximum_input_for_exact_output(
    output_amount: u64,
    input_reserve: u64,
    output_reserve: u64,
    liquidity_miner_fee: u64,
) -> u64 {
    assert(input_reserve > 0 && output_reserve > 0);
    let numerator = U128::from((0, input_reserve)) * U128::from((0, output_amount));
    let denominator = U128::from((
        0,
        calculate_amount_with_fee(output_reserve - output_amount, liquidity_miner_fee),
    ));
    let result_wrapped = (numerator / denominator).as_u64();

    if denominator > numerator {
        0 // 0 < result < 1, round the result down since there are no floating points
    } else {
        result_wrapped.unwrap() + 1
    }
}

/// Given exactly `input_amount` of the input asset, returns the minimum resulting amount of the output asset
pub fn minimum_output_given_exact_input(
    input_amount: u64,
    input_reserve: u64,
    output_reserve: u64,
    liquidity_miner_fee: u64,
) -> u64 {
    assert(input_reserve > 0 && output_reserve > 0);
    let input_amount_with_fee = calculate_amount_with_fee(input_amount, liquidity_miner_fee);
    let numerator = U128::from((0, input_amount_with_fee)) * U128::from((0, output_reserve));
    let denominator = U128::from((0, input_reserve)) + U128::from((0, input_amount_with_fee));
    let result_wrapped = (numerator / denominator).as_u64();
    result_wrapped.unwrap()
}

// Calculates d in the proportion a / b = c / d
pub fn proportional_value(b: u64, c: u64, a: u64) -> u64 {
    let calculation = (U128::from((0, b)) * U128::from((0, c)));
    let result_wrapped = (calculation / U128::from((0, a))).as_u64();
    result_wrapped.unwrap()
}

pub fn determine_assets(input_asset_id: ContractId, pair: Option<AssetPair>) -> (Asset, Asset) {
    require(pair.is_some(), InitError::AssetPairNotSet);
    let pair = pair.unwrap();
    require(input_asset_id == pair.a.id || input_asset_id == pair.b.id, InputError::InvalidAsset);
    (
        pair.this_asset(input_asset_id),
        pair.other_asset(input_asset_id),
    )
}
