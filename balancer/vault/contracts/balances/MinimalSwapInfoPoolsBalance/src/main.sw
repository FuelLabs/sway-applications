contract;

dep interface;
dep utils;

use interface::MinimalSwapInfoPoolsBalance;
use utils::vec_contains;

use std::{
    address::Address,
    storage::{StorageMap},
    vec::Vec,
    revert::{revert, require},
    constants::ZERO_B256,
};

use BalancerErrors::{TOKEN_ALREADY_REGISTERED, NONZERO_TOKEN_BALANCE, TOKEN_NOT_REGISTERED};
use BalanceAllocation::*;


storage {
    minimal_swap_info_pools_balances: StorageMap<(b256, Address), b256> = StorageMap { },
    minimal_swap_info_pools_tokens: StorageMap<b256, Vec<Address>> = StorageMap { },
}


impl MinimalSwapInfoPoolsBalance for Contract {
    #[storage(read, write)]
    fn register_minimal_swap_info_pool_tokens(pool_id: b256, tokens: Vec<Address>) {
        let mut pool_tokens = storage.minimal_swap_info_pools_tokens.get(pool_id);
        let mut count = 0;

        while(count < tokens.len()) {
            let token_contains = vec_contains(pool_tokens, tokens.get(count).unwrap());
            if !token_contains {
                pool_tokens.push(tokens.get(count).unwrap());
            } else {
                require(token_contains, TOKEN_ALREADY_REGISTERED);
            }
            count = count + 1;
        }
    }

    #[storage(read, write)]
    fn deregister_minimal_swap_info_pool_tokens(pool_id: b256, tokens: Vec<Address>) {
        let mut count = 0;

        while(count < tokens.len()) {
            let token: Address = tokens.get(count).unwrap();
            let balance = storage.minimal_swap_info_pools_balances.get((pool_id, token));
            require(is_zero(balance), NONZERO_TOKEN_BALANCE);

            // delete minimal_swap_info_pools_balances[poolId][token];
            // we need to delete the StorageMap in this case ^
            // but StorageMap does not have delete method on it
            // need to implemet it

            let token_removed = vec_remove_if_contains(pool_id, tokens.get(count).unwrap());
            if !token_removed {
                require(token_removed, TOKEN_NOT_REGISTERED);
            }
            count = count + 1;
        }
    }

    #[storage(read, write)]
    fn set_minimal_swap_info_pool_balances(pool_id: b256, tokens: Vec<Address>, balances: Vec<b256>) {
        let mut count = 0;

        while(count < tokens.len()) {
            let token: Address = tokens.get(count).unwrap();
            storage.minimal_swap_info_pools_balances.insert((pool_id, token), balances.get(count).unwrap());
            count = count + 1;
        }
    }

    #[storage(read, write)]
    fn minimal_swap_info_pool_cash_to_managed(pool_id: b256, token: Address, amount: u64) {
        update_minimal_swap_info_pool_balance_cash_to_managed(pool_id, token, amount);
    }

    #[storage(read, write)]
    fn minimal_swap_info_pool_managed_to_cash(pool_id: b256, token: Address, amount: u64) {
        update_minimal_swap_info_pool_balance_managed_to_cash(pool_id, token, amount);
    }

    #[storage(read, write)]
    fn set_minimal_swap_info_pool_managed_balance(pool_id: b256, token: Address, amount: u64) -> u64 {
        return update_minimal_swap_info_pool_balance_set_managed(pool_id, token, amount);
    }

    #[storage(read)]
    fn get_minimal_swap_info_pool_tokens(pool_id: b256) -> (Vec<Address>, Vec<b256>) {
        let pool_tokens = storage.minimal_swap_info_pools_tokens.get(pool_id);
        let mut tokens = ~Vec::new();
        let mut balances = ~Vec::new();

        let mut count = 0;
        while count < pool_tokens.len() {
            let token = pool_tokens.get(count).unwrap();
            tokens.push(token);
            balances.push(storage.minimal_swap_info_pools_balances.get((pool_id, token)));
            count = count + 1;
        }

        while count < pool_tokens.len() {
            tokens.push(~Address::from(ZERO_B256));
            count = count + 1;
        } 

        return (tokens, balances);
    }

    #[storage(read)]
    fn is_minimal_swap_info_pool_token_registered(pool_id: b256, token: Address) -> bool {
        let pool_tokens = storage.minimal_swap_info_pools_tokens.get(pool_id);
        
        return vec_contains(pool_tokens, token);
    }

    #[storage(write)]
    fn update_minimal_swap_info_pool_balances(pool_id: b256, token: Address, amount: b256) {
        storage.minimal_swap_info_pools_balances.insert((pool_id, token), amount);
    }

    #[storage(read)]
    fn external_get_minimal_swap_info_pool_balance(pool_id: b256, token: Address) -> b256 {
        return get_minimal_swap_info_pool_balance(pool_id, token); 
    }

}


// helping function
#[storage(read, write)]
fn vec_remove_if_contains(pool_id: b256, delete: Address) -> bool {
    let mut vec = storage.minimal_swap_info_pools_tokens.get(pool_id);
    let mut count = 0;
    let mut return_bool: bool = true;

    while(count < vec.len()) {
        if vec.get(count).unwrap() == delete {
            vec.remove(count);
            return_bool =  true;
        } else {
            return_bool =  true;
        }
        count = count + 1;
    }

    return return_bool;
}

#[storage(read)]
fn get_minimal_swap_info_pool_balance(pool_id: b256, token: Address) -> b256 {
    let balance = storage.minimal_swap_info_pools_balances.get((pool_id, token));
    let token_registered = is_zero(balance) || vec_contains(storage.minimal_swap_info_pools_tokens.get(pool_id), token);

    if(!token_registered) {
        // PoolRegistry::_ensure_registered_pool(pool_id);
        revert(TOKEN_NOT_REGISTERED);
    }

    return balance;
}

// returns signed int, but we dont have in sway
#[storage(read, write)]
fn update_minimal_swap_info_pool_balance_cash_to_managed(pool_id: b256, token: Address, amount: u64) -> u64 {
    let current_balance = get_minimal_swap_info_pool_balance(pool_id, token);
    let new_balance = cash_to_managed(current_balance, amount);
    storage.minimal_swap_info_pools_balances.insert((pool_id, token), new_balance);

    return managed_delta(new_balance, current_balance);
}

// returns signed int, but we dont have in sway
#[storage(read, write)]
fn update_minimal_swap_info_pool_balance_managed_to_cash(pool_id: b256, token: Address, amount: u64) -> u64 {
    let current_balance = get_minimal_swap_info_pool_balance(pool_id, token);
    let new_balance = managed_to_cash(current_balance, amount);
    storage.minimal_swap_info_pools_balances.insert((pool_id, token), new_balance);

    return managed_delta(new_balance, current_balance);
}

// returns signed int, but we dont have in sway
#[storage(read, write)]
fn update_minimal_swap_info_pool_balance_set_managed(pool_id: b256, token: Address, amount: u64) -> u64 {
    let current_balance = get_minimal_swap_info_pool_balance(pool_id, token);
    let new_balance = set_managed(current_balance, amount);
    storage.minimal_swap_info_pools_balances.insert((pool_id, token), new_balance);

    return managed_delta(new_balance, current_balance);
}