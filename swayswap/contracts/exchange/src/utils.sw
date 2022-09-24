library utils;

use core::num::*;
use std::{
    assert::assert,
    chain::auth::{
        AuthError,
        msg_sender,
    },
    contract_id::ContractId,
    result::Result,
    revert::revert,
    storage::{
        get,
        store,
    },
    u128::U128,
};

/// Token ID of Ether
pub const ETH_ID = 0x0000000000000000000000000000000000000000000000000000000000000000;

/// The token ID key from storage
/// Contract ID of the token on the other side of the pool
/// Set of the deploy time
pub const TOKEN_ID_KEY = 0x0000000000000000000000000000000000000000000000000000000000000001;

/// Minimum ETH liquidity to open a pool
pub const MINIMUM_LIQUIDITY = 1; // A more realistic value would be 1000000000
// Liquidity miner fee apply to all swaps
pub const LIQUIDITY_MINER_FEE = 333;

/// Return token reserve balance
#[storage(read)]
pub fn get_current_reserve(token_id: b256) -> u64 {
    get::<u64>(token_id)
}

/// Add amount to the token reserve
#[storage(read, write)]
pub fn add_reserve(token_id: b256, amount: u64) {
    let value = get::<u64>(token_id);
    store(token_id, value + amount);
}

/// Remove amount to the token reserve
#[storage(read, write)]
pub fn remove_reserve(token_id: b256, amount: u64) {
    let value = get::<u64>(token_id);
    store(token_id, value - amount);
}

// Calculate 0.3% fee
pub fn calculate_amount_with_fee(amount: u64) -> u64 {
    let fee: u64 = (amount / LIQUIDITY_MINER_FEE);
    amount - fee
}

pub fn mutiply_div(a: u64, b: u64, c: u64) -> u64 {
    let calculation = (~U128::from(0, a) * ~U128::from(0, b));
    let result_wrapped = (calculation / ~U128::from(0, c)).as_u64();
    result_wrapped.unwrap()
}

pub fn div_mutiply(a: u64, b: u64, c: u64) -> u64 {
    let calculation = (~U128::from(0, a) / ~U128::from(0, b));
    let result_wrapped = (calculation * ~U128::from(0, c)).as_u64();
    result_wrapped.unwrap()
}

/// Pricing function for converting between ETH and Tokens.
pub fn get_input_price(input_amount: u64, input_reserve: u64, output_reserve: u64) -> u64 {
    assert(input_reserve > 0 && output_reserve > 0);
    let input_amount_with_fee: u64 = calculate_amount_with_fee(input_amount);
    let numerator = ~U128::from(0, input_amount_with_fee) * ~U128::from(0, output_reserve);
    let denominator = ~U128::from(0, input_reserve) + ~U128::from(0, input_amount_with_fee);
    let result_wrapped = (numerator / denominator).as_u64();
    result_wrapped.unwrap()
}

/// Pricing function for converting between ETH and Tokens.
pub fn get_output_price(output_amount: u64, input_reserve: u64, output_reserve: u64) -> u64 {
    assert(input_reserve > 0 && output_reserve > 0);
    let numerator = ~U128::from(0, input_reserve) * ~U128::from(0, output_amount);
    let denominator = ~U128::from(0, calculate_amount_with_fee(output_reserve - output_amount));
    let result_wrapped = (numerator / denominator).as_u64();
    if denominator > numerator {
        // Emulate Infinity Value
        ~u64::max()
    } else {
        result_wrapped.unwrap() + 1
    }
}
