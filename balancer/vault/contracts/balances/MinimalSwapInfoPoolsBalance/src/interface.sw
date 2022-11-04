library interface;

use std::{
    address::Address,
    vec::Vec,
};


abi MinimalSwapInfoPoolsBalance {
    #[storage(read, write)]fn register_minimal_swap_info_pool_tokens(pool_id: b256, tokens: Vec<Address>);
    #[storage(read, write)]fn deregister_minimal_swap_info_pool_tokens(pool_id: b256, tokens: Vec<Address>);
    #[storage(read, write)]fn set_minimal_swap_info_pool_balances(pool_id: b256, tokens: Vec<Address>, balances: Vec<b256>);
    #[storage(read, write)]fn minimal_swap_info_pool_cash_to_managed(pool_id: b256, token: Address, amount: u64);
    #[storage(read, write)]fn minimal_swap_info_pool_managed_to_cash(pool_id: b256, token: Address, amount: u64);
    // // returns a signed int though, but we dont have signed int in sway
    #[storage(read, write)]fn set_minimal_swap_info_pool_managed_balance(pool_id: b256, token: Address, amount: u64) -> u64;
    #[storage(read)]fn get_minimal_swap_info_pool_tokens(pool_id: b256) -> (Vec<Address>, Vec<b256>);
    #[storage(read)]fn is_minimal_swap_info_pool_token_registered(pool_id: b256, token: Address) -> bool;
    #[storage(write)]fn update_minimal_swap_info_pool_balances(pool_id: b256, token: Address, amount: b256);
    #[storage(read)]fn external_get_minimal_swap_info_pool_balance(pool_id: b256, token: Address) -> b256;
}