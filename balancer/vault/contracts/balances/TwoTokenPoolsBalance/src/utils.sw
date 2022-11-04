library utils;

dep data_structures;

use data_structures::abi_encode;

use std::{
    address::Address,
    hash::keccak256,
};

pub fn get_two_token_pair_hash(token_a: Address, token_b: Address) -> b256 {
    let tmp = abi_encode {
        token_a: token_a,
        token_b: token_b,
    };
    return keccak256(tmp);
}

pub fn sort_two_tokens(token_x: Address, token_y: Address) -> (Address, Address) {
    let token_a: b256 = token_x.into();
    let token_b: b256 = token_y.into();
    if token_a < token_b {
        return (token_x, token_y);
    } 
    return (token_y, token_x);
}