library BalanceAllocation;

use std::{revert::require, option::Option, vec::Vec};

use BalancerErrors::*;

// changes/doubts -> refer to vault/contracts/balances/BalanceAllocation.sol

/// Extract a single 64 bit word from a b256 value using the specified offset.
fn get_word_from_b256(val: b256, offset: u64) -> u64 {
    let mut empty: u64 = 0;
    asm(r1: val, offset: offset, r2, res: empty) {
        add r2 r1 offset;
        lw res r2 i0;
        res: u64
    }
}

/// Build a single b256 value from 4 64 bit words.
fn compose(word_1: u64) -> b256 {
    let res: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    asm(w1: word_1, result: res) {
        sw result w1 i0;
        result: b256
    }
}


// max function
fn max(first: u64, second: u64) -> u64 {
    if first > second {
        return first;
    } else {
        return second;
    }
}

const MASK: u64 = 1;

pub fn total(balance: b256) -> u64 {
    // return cash(balance) + managed(balance);
    return(cash(balance) + managed(balance));
}

pub fn cash(balance: b256) -> u64 {
    // let mask: u64 = 2**(112) - 1;

    return(get_word_from_b256(balance, 0) & MASK);
}

pub fn managed(balance: b256) -> u64 {
    // let mask: u64 = 2**(112) - 1;

    return((get_word_from_b256(balance, 0) >> 112) & MASK);
}

pub fn last_change_block(balance: b256) -> u64 {
    // let mask: u64 = 2**(32) - 1;
    return((get_word_from_b256(balance, 0) >> 224) & MASK);
}

pub fn managed_delta(new_balance: b256, old_balance: b256) -> u64 {
    return(managed(new_balance) - managed(old_balance));
}

// change/doubts -> we might need to use `insert` method on vec, instead of push
pub fn totals_and_last_change_block(balances: Vec<b256>) -> (Vec<u64>, u64) {
    let mut i = 0;
    let mut results = ~Vec::new();
    let mut last_change_block_ = 0;

    while(i < results.len()) {
        let balance = balances.get(i).unwrap();
        results.push(total(balance));
        // results.insert(i, total(balance));
        last_change_block_ = max(last_change_block_, last_change_block(balance));

        i += 1;
    }

    return(results, last_change_block_);
}

pub fn is_zero(balance: b256) -> bool {
    // let mask: u64 = 2**(224) - 1;

    return(get_word_from_b256(balance, 0) & MASK) == 0;
}

pub fn is_not_zero(balance: b256) -> bool {
    return !is_zero(balance);
}

pub fn to_balance(_cash: u64, _managed: u64, _block_number: u64) -> b256 {
    let _total: u64 = _cash + _managed;

    // mask here -> let mask: u64 = 2**112;
    require(_total >= _cash && _total < MASK, BALANCE_TOTAL_OVERFLOW);

    return pack(_cash, _managed, _block_number);
}

pub fn increase_cash(balance: b256, amount: u64) -> b256 {
    // see if there is any checked_add() on u64 types, use that if comes in the future
    let new_cash: u64 = cash(balance) + amount;
    let current_managed: u64 = managed(balance);
    // let new_last_change_block: u64 = block.number;
    let new_last_change_block: u64 = 22;

    return to_balance(new_cash, current_managed, new_last_change_block);
}

pub fn decrease_cash(balance: b256, amount: u64) -> b256 {
    // see if there is any checked_sub() on u64 types, use that if comes in the future
    let new_cash: u64 = cash(balance) - amount;
    let current_managed: u64 = managed(balance);
    // let new_last_change_block: u64 = block.number;
    let new_last_change_block: u64 = 22;

    return to_balance(new_cash, current_managed, new_last_change_block);
}

pub fn cash_to_managed(balance: b256, amount: u64) -> b256 {
    // see if there is any checked_sub() on u64 types, use that if comes in the future
    let new_cash: u64 = cash(balance) - amount;
    // see if there is any checked_add() on u64 types, use that if comes in the future
    let new_managed: u64 = managed(balance) + amount;
    let current_last_change_block: u64 = last_change_block(balance);

    return to_balance(new_cash, new_managed, current_last_change_block);
}

pub fn managed_to_cash(balance: b256, amount: u64) -> b256 {
    // see if there is any checked_add() on u64 types, use that if comes in the future
    let new_cash: u64 = cash(balance) + amount;
    // see if there is any checked_sub() on u64 types, use that if comes in the future
    let new_managed: u64 = managed(balance) - amount;
    let current_last_change_block: u64 = last_change_block(balance);

    return to_balance(new_cash, new_managed, current_last_change_block);
}

pub fn set_managed(balance: b256, new_managed: u64) -> b256 {
    let current_cash: u64 = cash(balance);
    // let new_last_change_block: u64 = block.number;
    let new_last_change_block: u64 = 22;

    return to_balance(current_cash, new_managed, new_last_change_block);
}

pub fn decode_balance_a(shared_balance: b256) -> u64 {
    // let mask: u64 = 2**(112) - 1;

    return(get_word_from_b256(shared_balance, 0) & MASK);
}

pub fn decode_balance_b(shared_balance: b256) -> u64 {
    // let mask: u64 = 2**(112) - 1;

    return((get_word_from_b256(shared_balance, 0) >> 112) & MASK);
}

pub fn from_shared_to_balance_a(shared_cash: b256, shared_managed: b256) -> b256 {
    return to_balance(decode_balance_a(shared_cash), decode_balance_a(shared_managed), last_change_block(shared_cash));
}

pub fn from_shared_to_balance_b(shared_cash: b256, shared_managed: b256) -> b256 {
    return to_balance(decode_balance_b(shared_cash), decode_balance_b(shared_managed), last_change_block(shared_cash));
}

pub fn to_shared_cash(token_a_balance: b256, token_b_balance: b256) -> b256 {
    let new_last_change_block: u64 = max(last_change_block(token_a_balance), last_change_block(token_b_balance));

    return pack(cash(token_a_balance), cash(token_b_balance), new_last_change_block);
}

pub fn to_shared_managed(token_a_balance: b256, token_b_balance: b256) -> b256 {
    return pack(managed(token_a_balance), managed(token_b_balance), 0);
}

pub fn pack(_least_significant: u64, _mid_significant: u64, _most_significant: u64) -> b256 {
    return(compose((_most_significant << 224) + (_mid_significant << 112) + _least_significant));
}
