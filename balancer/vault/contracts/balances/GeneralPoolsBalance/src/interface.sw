library interface;

use std::{
    address::Address,
    vec::Vec,
};

abi GeneralPoolsBalance {
    #[storage(write, read)]fn _register_general_pool_tokens(pool_id: b256, tokens: Vec<Address>);
    #[storage(write, read)]fn _deregister_general_pool_tokens(pool_id: b256, tokens: Vec<Address>);
    #[storage(write, read)]fn _set_general_pool_balances(pool_id: b256, balances: Vec<u64>);
    #[storage(write, read)]fn _general_pool_cash_to_managed(pool_id: b256, token: Address, amount: u64);
    #[storage(write, read)]fn _general_pool_managed_to_cash(pool_id: b256, token: Address, amount: u64);
    // this function returns signed int in solidity code
    #[storage(write, read)]fn _set_general_pool_managed_balance(pool_id: b256, token: Address, amount: u64) -> u64;
    #[storage(read)]fn _get_general_pool_tokens(pool_id: b256) -> (Vec<Address>, Vec<u64>);
    #[storage(read)]fn _is_general_pool_token_registered(pool_id: b256, token: Address) -> bool;
}
