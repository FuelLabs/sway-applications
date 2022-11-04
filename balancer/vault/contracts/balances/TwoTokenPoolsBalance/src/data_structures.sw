library data_structures;

use std::address::Address;


pub const ZERO_ADDRESS: Address = ~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000);
pub const TOKEN_NOT_REGISTERED = 512;
pub struct abi_encode {
    token_a: Address,
    token_b: Address,
}

pub struct TwoTokenPoolBalances {
    shared_cash: b256,
    shared_managed: b256
}

pub struct TwoTokenPoolTokens {
    token_a: Address,
    token_b: Address,
    // workaround of nested storageMap
    // balances: StorageMap<b256, TwoTokenPoolBalances>,
    balances: b256,
}