contract;

dep data_structures;
dep errors;
dep interface;
dep utils;

use data_structures::{
    abi_encode,
    TwoTokenPoolBalances,
    TwoTokenPoolTokens,
    ZERO_ADDRESS,
    TOKEN_NOT_REGISTERED,
};
use errors::Error;
use interface::TwoTokenPoolsBalance;
use utils::{get_two_token_pair_hash, sort_two_tokens};

use std::{
    address::Address,
    vec::Vec,
    storage::StorageMap,
    hash::keccak256,
    revert::{revert, require},
    contract_id::ContractId,
};

use BalanceAllocation::{
    managed_delta,
    is_zero,
    cash_to_managed,
    managed_to_cash,
    set_managed,
    from_shared_to_balance_a,
    to_shared_cash,
    to_shared_managed,
    from_shared_to_balance_b,
};
// use PoolRegistry::PoolRegistry;

storage {
    two_token_pool_tokens: StorageMap<b256, TwoTokenPoolTokens> = StorageMap { },
    // first b256 value is the pool id sencond b256 value is balances
    balances: StorageMap<(b256, b256), TwoTokenPoolBalances> = StorageMap {},
    pool_registry_contract_id: ContractId = ContractId{ value: 0x0000000000000000000000000000000000000000000000000000000000000000,},
}


impl TwoTokenPoolsBalance for Contract {
    #[storage(read, write)]
    fn deregister_two_token_pool_tokens(pool_id: b256, token_x: Address, token_y: Address) {
        let (balance_a, balance_b) = get_two_token_pool_shared_balances(pool_id, token_x, token_y);

        require(is_zero(balance_a) && is_zero(balance_b), Error::NONZERO_TOKEN_BALANCE);

        // delete _twoTokenPoolTokens[poolId];
        // delete poolBalances.sharedCash;
        
        // No delete methods for storage yet
        // need to implement it, as soon as we get support for that
    }

    #[storage(read, write)]
    fn register_two_token_pool_tokens(pool_id: b256, token_x: Address, token_y: Address) {
        require(token_x != token_y, Error::TOKEN_ALREADY_REGISTERED);
        let token_a: b256 = token_x.into();
        let token_b: b256 = token_y.into();
        require(token_a < token_b, Error::UNSORTED_TOKENS);

        let mut pool_tokens = storage.two_token_pool_tokens.get(pool_id);
        require(pool_tokens.token_a == ZERO_ADDRESS && pool_tokens.token_a == ZERO_ADDRESS, Error::TOKENS_ALREADY_SET);

        pool_tokens.token_a = token_x;
        pool_tokens.token_b = token_y;
    }


    #[storage(read, write)]
    fn set_two_token_pool_cash_balances(pool_id: b256, token_a: Address, balance_a: b256, token_b: Address, balance_b: b256) {
        let pair_hash = get_two_token_pair_hash(token_a, token_b);
        let pool_balances = storage.two_token_pool_tokens.get(pool_id).balances;
        let mut balalnce = storage.balances.get((pool_id, pool_balances));
        balalnce.shared_cash = to_shared_cash(balance_a, balance_b);

        let bal = storage.two_token_pool_tokens.get(pool_id).balances;
        storage.balances.insert((pool_id, pair_hash), balalnce);
    }

    #[storage(read, write)]
    fn two_token_pool_cash_to_managed(pool_id: b256, token: Address, amount: u64) {
        update_two_token_pool_shared_balance_cash_to_managed(pool_id, token, amount);
    }

    #[storage(read, write)]
    fn two_token_pool_managed_to_cash(pool_id: b256, token: Address, amount: u64) {
        update_two_token_pool_shared_balance_managed_to_cash(pool_id, token, amount);
    }

    #[storage(read, write)]
    fn set_two_token_pool_managed_balance(pool_id: b256, token: Address, amount: u64) -> u64 {
        return update_two_token_pool_shared_balance_set_managed(pool_id, token, amount);
    }

    #[storage(read)]
    fn get_two_token_pool_tokens(pool_id: b256) -> (Vec<Address>, Vec<b256>) {
        let (token_a, balance_a, token_b, balance_b) = get_two_token_pool_balances(pool_id);

        if token_a == ZERO_ADDRESS || token_b == ZERO_ADDRESS {
            let add: Vec<Address> = ~Vec::new();
            let byt: Vec<b256> = ~Vec::new();
            return (add, byt);
        }

        let mut tokens = ~Vec::new();
        tokens.push(token_a);
        tokens.push(token_b);

        let mut balances = ~Vec::new();
        balances.push(balance_a);
        balances.push(balance_b);

        return (tokens, balances);
    }

    #[storage(read)]
    fn get_two_token_pool_balance(pool_id: b256, token: Address) -> b256 {
        let (token_a, balance_a, token_b, balance_b) = get_two_token_pool_balances(pool_id);

        if token == token_a {
            return balance_a;
        } else if token == token_b {
            return balance_b;
        } else {
            // revert(TOKEN_NOT_REGISTERED);
            return 0x0000000000000000000000000000000000000000000000000000000000000000;
        }
    }
}


#[storage(read)]
fn get_two_token_pool_balances(pool_id: b256) -> (Address, b256, Address, b256) {
    let pool_tokens = storage.two_token_pool_tokens.get(pool_id);
    let token_a: Address = pool_tokens.token_a;
    let token_b: Address = pool_tokens.token_b;

    let pair_hash = get_two_token_pair_hash(token_a, token_b);
    let pool_balances = pool_tokens.balances;

    let shared_cash = storage.balances.get((pool_id, pool_balances)).shared_cash;
    let shared_managed = storage.balances.get((pool_id, pool_balances)).shared_managed;

    let balance_a = from_shared_to_balance_a(shared_cash, shared_managed);
    let balance_b = from_shared_to_balance_b(shared_cash, shared_managed);

    return (token_a, balance_a, token_b, balance_b);
}

#[storage(read)]
fn get_two_token_pool_shared_balances(pool_id: b256, token_x: Address, token_y: Address) -> (b256, b256) {
    let (token_a, token_b) = sort_two_tokens(token_x, token_y);
    let pair_hash = get_two_token_pair_hash(token_a, token_b);

    let pool_balances = storage.two_token_pool_tokens.get(pool_id).balances;

    let shared_cash = storage.balances.get((pool_id, pool_balances)).shared_cash;
    let shared_managed = storage.balances.get((pool_id, pool_balances)).shared_managed;

    let token_registered = is_zero(shared_cash) || is_zero(shared_managed) || (is_two_token_pool_token_registered(pool_id, token_a) && is_two_token_pool_token_registered(pool_id, token_b));

    if !token_registered {
        //todo need to check
        // let x = abi(PoolRegistry, pool_registry_contract_id);
        // x._ensure_registered_pool(pool_id);
        revert(TOKEN_NOT_REGISTERED);
        // return (0x0000000000000000000000000000000000000000000000000000000000000000, 0x0000000000000000000000000000000000000000000000000000000000000000);
    }

    let balance_a = from_shared_to_balance_a(shared_cash, shared_managed);
    let balance_b = from_shared_to_balance_b(shared_cash, shared_managed);

    return (balance_a, balance_b);
}

#[storage(read)]
fn is_two_token_pool_token_registered(pool_id: b256, token: Address) -> bool {
    let pool_tokens = storage.two_token_pool_tokens.get(pool_id);
    return (token == pool_tokens.token_a || token == pool_tokens.token_b) && token != ZERO_ADDRESS;
}

// todo returns signed int in solidity. no support of signed int in sway
#[storage(read, write)]
fn update_two_token_pool_shared_balance_cash_to_managed(pool_id: b256, token: Address, amount: u64) -> u64 {
    let(token_a, mut balance_a, token_b, mut balance_b) = get_two_token_pool_balances(pool_id);

    let mut delta: u64 = 0;

    if token == token_a {
        let new_balance = cash_to_managed(balance_a, amount);
        delta = managed_delta(new_balance, balance_a);
        balance_a = new_balance;
    } else {
        let new_balance = cash_to_managed(balance_b, amount);
        delta = managed_delta(new_balance, balance_b);
        balance_b = new_balance;
    }

    let balance = TwoTokenPoolBalances {
        shared_cash: to_shared_cash(balance_a, balance_b),
        shared_managed: to_shared_managed(balance_a, balance_b),
    };
 
    let pair_hash = get_two_token_pair_hash(token_a, token_b);
    let bal = storage.two_token_pool_tokens.get(pool_id).balances;
    storage.balances.insert((pool_id, pair_hash), balance);

    return delta;
}

// todo returns signed int in solidity. no support of signed int in sway
#[storage(read, write)]
fn update_two_token_pool_shared_balance_managed_to_cash(pool_id: b256, token: Address, amount: u64) -> u64 {
    let(mut token_a, mut balance_a, token_b, mut balance_b) = get_two_token_pool_balances(pool_id);

    let mut delta: u64 = 0;

    if token_a == token_b {
        let new_balance = managed_to_cash(balance_a, amount);
        delta = managed_delta(new_balance, balance_a);
        balance_a = new_balance;
    } else {
        let new_balance = cash_to_managed(balance_b, amount);
        delta = managed_delta(new_balance, balance_b);
        balance_b = new_balance;
    }

    let balance = TwoTokenPoolBalances {
        shared_cash: to_shared_cash(balance_a, balance_b),
        shared_managed: to_shared_managed(balance_a, balance_b),
    };
 
    let pair_hash = get_two_token_pair_hash(token_a, token_b);
    let bal = storage.two_token_pool_tokens.get(pool_id).balances;
    storage.balances.insert((pool_id, pair_hash), balance);

    return delta;
}

#[storage(read, write)]
fn update_two_token_pool_shared_balance_set_managed(pool_id: b256, token: Address, amount: u64) -> u64 {
    let(mut token_a, mut balance_a, token_b, mut balance_b) = get_two_token_pool_balances(pool_id);
    let mut delta: u64 = 0;

    if token_a == token_b {
        let new_balance = set_managed(balance_a, amount);
        delta = managed_delta(new_balance, balance_a);
        balance_a = new_balance;
    } else {
        let new_balance = cash_to_managed(balance_b, amount);
        delta = managed_delta(new_balance, balance_b);
        balance_b = new_balance;
    }

    let balance = TwoTokenPoolBalances {
        shared_cash: to_shared_cash(balance_a, balance_b),
        shared_managed: to_shared_managed(balance_a, balance_b),
    };
 
    let pair_hash = get_two_token_pair_hash(token_a, token_b);
    let bal = storage.two_token_pool_tokens.get(pool_id).balances;
    storage.balances.insert((pool_id, pair_hash), balance);

    return delta;
}

