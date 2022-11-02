library utils;

use core::num::*;
use std::u128::U128;

fn calculate_amount_with_fee(amount: u64, liquidity_miner_fee: u64) -> u64 {
    let fee = (amount / liquidity_miner_fee);
    amount - fee
}

pub fn div_multiply(a: u64, b: u64, c: u64) -> u64 {
    let calculation = (~U128::from(0, a) / ~U128::from(0, b));
    let result_wrapped = (calculation * ~U128::from(0, c)).as_u64();
    result_wrapped.unwrap()
}

/// Returns the maximum required amount of the input asset to get exactly ` output_amount ` of the output asset
pub fn get_maximum_input_for_exact_output(
    output_amount: u64,
    input_reserve: u64,
    output_reserve: u64,
    liquidity_miner_fee: u64,
) -> u64 {
    assert(input_reserve > 0 && output_reserve > 0);
    let numerator = ~U128::from(0, input_reserve) * ~U128::from(0, output_amount);
    let denominator = ~U128::from(0, calculate_amount_with_fee(output_reserve - output_amount, liquidity_miner_fee));
    let result_wrapped = (numerator / denominator).as_u64();

    if denominator > numerator {
        ~u64::max()
    } else {
        result_wrapped.unwrap() + 1
    }
}

/// Given exactly ` input_amount ` of the input asset, returns the minimum resulting amount of the output asset
pub fn get_minimum_output_given_exact_input(
    input_amount: u64,
    input_reserve: u64,
    output_reserve: u64,
    liquidity_miner_fee: u64,
) -> u64 {
    assert(input_reserve > 0 && output_reserve > 0);
    let input_amount_with_fee = calculate_amount_with_fee(input_amount, liquidity_miner_fee);
    let numerator = ~U128::from(0, input_amount_with_fee) * ~U128::from(0, output_reserve);
    let denominator = ~U128::from(0, input_reserve) + ~U128::from(0, input_amount_with_fee);
    let result_wrapped = (numerator / denominator).as_u64();
    result_wrapped.unwrap()
}

pub fn multiply_div(a: u64, b: u64, c: u64) -> u64 {
    let calculation = (~U128::from(0, a) * ~U128::from(0, b));
    let result_wrapped = (calculation / ~U128::from(0, c)).as_u64();
    result_wrapped.unwrap()
}
