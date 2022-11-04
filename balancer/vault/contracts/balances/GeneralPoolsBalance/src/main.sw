contract;

dep data_structures;
dep interface;
dep errors;

use data_structures::{
    TOKEN_ALREADY_REGISTERED,
    IERC20ToBytes32MapEntry,
    IERC20ToBytes32Map,
};
use interface::GeneralPoolsBalance;
use errors::Error;

use std::{
    storage::StorageMap,
    address::Address,
    vec::Vec,
    option::Option,
    revert::{revert, require},
    contract_id::ContractId,
};

use BalanceAllocation::*;

storage {
    general_pools_balances: StorageMap<b256, IERC20ToBytes32Map> = StorageMap{ },
    entries: StorageMap<(b256, u64), IERC20ToBytes32MapEntry> = StorageMap{ },
    indexes: StorageMap<(b256, ContractId), u64> = StorageMap{ },
}

impl GeneralPoolsBalance for Contract {
    #[storage(write, read)]
    fn _register_general_pool_tokens(pool_id: b256, tokens: Vec<Address>) {
        let pool_balances = storage.general_pools_balances.get(pool_id);
        let mut i = 0;
        // let zer_address: b256 = ;
        while(i < tokens.len()) {
            let token: Address = tokens.get(i).unwrap();
            if get(pool_id, ~ContractId::from(token.into())) == 0x0000000000000000000000000000000000000000000000000000000000000000 {
                revert(TOKEN_ALREADY_REGISTERED);
            } else {
                let token = ~ContractId::from(token.into());
                set(pool_id, (pool_id, token), 0x0000000000000000000000000000000000000000000000000000000000000000);
            }


            i += 1;

        }
    }

    #[storage(write, read)]
    fn _deregister_general_pool_tokens(pool_id: b256, tokens: Vec<Address>) {
        let pool_balances = storage.general_pools_balances.get(pool_id);
        let mut i = 0;

        while(i < tokens.len()) {
            let token: Address = tokens.get(i).unwrap();
            let current_balance = _get_general_pool_balance_storage_map(pool_id, token);

            require(current_balance == 0x0000000000000000000000000000000000000000000000000000000000000000, Error::NONZERO_TOKEN_BALANCE);

            // pool_balances.remove(token);
            // no remove method on StorageMap
            // so assigning 0 to token
            let token = ~ContractId::from(token.into());
            set(pool_id, (pool_id, token), 0x0000000000000000000000000000000000000000000000000000000000000000);

            i += 1;
        }
    }

    #[storage(write, read)]
    fn _set_general_pool_balances(pool_id: b256, balances: Vec<u64>) {
        let pool_balances = storage.general_pools_balances.get(pool_id);
        let mut i = 0;

        while(i < balances.len()) {
            // poolBalances.unchecked_setAt(i, balances[i]);
            // this unchecked_setAt() method sets value of k-v pair using indexes
            // but we dont have StorageMap index, so this is not implemented currently
            
            // pool_balances.insert()

            i += 1;
        }      
    }

    #[storage(write, read)]
    fn _general_pool_cash_to_managed(pool_id: b256, token: Address, amount: u64) {
        _update_general_pool_balance_cash_to_managed(pool_id, token, amount);
    }

    #[storage(write, read)]
    fn _general_pool_managed_to_cash(pool_id: b256, token: Address, amount: u64) {
        _update_general_pool_balance_managed_to_cash(pool_id, token, amount);
    }

    #[storage(write, read)]
    fn _set_general_pool_managed_balance(pool_id: b256, token: Address, amount: u64) -> u64 {
        return _update_general_pool_balance_set_managed(pool_id, token, amount);
    }

    #[storage(read)]
    fn _get_general_pool_tokens(pool_id: b256) -> (Vec<Address>, Vec<u64>) {
        let pool_balances = storage.general_pools_balances.get(pool_id);
        let mut i = 0;

        // EnumerableMap.IERC20ToBytes32Map storage poolBalances = _generalPoolsBalances[poolId];
        // tokens = new IERC20[](poolBalances.length());

        // no way to get StorageMap length, like what is used in solidity ^
        // using a dummy len for now
        // let mut tokens: Vec<Address> = Vec::with_capacity(pool_balances.len());
        let mut tokens: Vec<Address> = ~Vec::with_capacity(50);
        let mut balances: Vec<u64> = ~Vec::with_capacity(50);
        while(i < tokens.len()) {
            //(tokens[i], balances[i]) = poolBalances.unchecked_at(i);
            // no way to get key-value from StorageMap with the help of indexes 
            // like what is used here in Solidity's approach here ^

            // tokens.insert(i, token_here);
            // balances.insert(i, balance_here);

            // need to fix this ^ if we find a workaround

            i += 1;
        }

        return (tokens, balances);
    }

    #[storage(read)]
    fn _is_general_pool_token_registered(pool_id: b256, token: Address) -> bool {
        let pool_balances = storage.general_pools_balances.get(pool_id);
        // return poolBalances.contains(token);
        // ^ no contains method for StorageMap
        // need to replace this after support

        return true;
    }

}

#[storage(read, write)]
fn set(pool_id: b256, key: (b256, ContractId), value: b256) -> bool {  
    // We read and store the key's index to prevent multiple reads from the same storage slot
    let keyIndex = storage.indexes.get(key);
    let mut pool_balances = storage.general_pools_balances.get(pool_id);

    // Equivalent to !contains(map, key)
    if keyIndex == 0 {
        let previousLength = pool_balances.length;
        let tmp = IERC20ToBytes32MapEntry{ 
            _key: key, 
            _value: value 
        };
        storage.entries.insert((pool_id, previousLength), tmp);
        pool_balances.length = previousLength + 1;
        // The entry is stored at previousLength, but we add 1 to all indexes
        // and use 0 as a sentinel value
        storage.indexes.insert(key, previousLength + 1);
        return true;
    }
    else {
        let tmp = IERC20ToBytes32MapEntry{ 
            _key: storage.entries.get((pool_id, keyIndex - 1))._key, 
            _value: value 
        };
        storage.entries.insert((pool_id, keyIndex - 1), tmp);
        return false;
    }
}

#[storage(read, write)]
fn unchecked_set_at(pool_id: b256, index: u64, value: b256) {
    let mut entry = storage.entries.get((pool_id, index));
    entry._value = value;
    storage.entries.insert((pool_id, index), entry);
}

#[storage(read, write)]
fn remove(pool_id: b256, key: ContractId) -> bool {
    let key_index = storage.indexes.get((pool_id, key));

    if key_index != 0 {
        let to_delete_index = key_index - 1;
        let last_index = storage.general_pools_balances.get(pool_id).length - 1;
        
        if to_delete_index != last_index {
            let last_entry = storage.entries.get((pool_id, last_index));
            storage.entries.insert((pool_id, to_delete_index), last_entry);
            storage.indexes.insert(last_entry._key, to_delete_index + 1);
        }

        // setting the value to 0 or equivalent
        storage.entries.insert((pool_id, last_index), IERC20ToBytes32MapEntry {
            _key: (pool_id, ~ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000)),
            _value: 0x0000000000000000000000000000000000000000000000000000000000000000
        });

        // map._length = lastIndex;
        let mut tmp = storage.general_pools_balances.get(pool_id).length;
        tmp = last_index;

        // delete map._indexes[key];
        storage.indexes.insert((pool_id, key), 0);

        return true;
    } else {
        return false;
    }
}

#[storage(read)]
fn contains(pool_id: b256, key: ContractId) -> bool {
    return storage.indexes.get((pool_id, key)) != 0;
}

#[storage(read)]
fn length(pool_id: b256) -> u64 {
    return storage.general_pools_balances.get(pool_id).length;
}

#[storage(read)]
fn at(pool_id: b256, index: u64) -> ((b256, ContractId), b256) {
    let pools_balance = storage.general_pools_balances.get(pool_id);
    require(pools_balance.length > index, Error::OUT_OF_BOUNDS);
    return unchecked_at(pool_id, index);
}

#[storage(read)]
fn unchecked_at(pool_id: b256, index: u64) -> ((b256, ContractId), b256) {
    let entry = storage.entries.get((pool_id, index));
    return (entry._key, entry._value);
}

#[storage(read)]
fn unchecked_value_at(pool_id: b256, index: u64) -> b256 {
    return storage.entries.get((pool_id, index))._value;
}

#[storage(read)]
fn get(pool_id: b256, key: ContractId) -> b256 {
    let index = storage.indexes.get((pool_id, key));
    require(index > 0, Error::OUT_OF_BOUNDS);
    return unchecked_value_at(pool_id, index - 1);
}

#[storage(read)]
fn index_of(pool_id: b256, key: ContractId) -> u64 {
    let unchecked_index = unchecked_index_of(pool_id, key);
    require(unchecked_index != 0, Error::OUT_OF_BOUNDS);
    return unchecked_index - 1;
}

#[storage(read)]
fn unchecked_index_of(pool_id: b256, key: ContractId) -> u64 {
    return storage.indexes.get((pool_id, key));
}

// this function returns signed int in solidity code
#[storage(write, read)]
fn _update_general_pool_balance_cash_to_managed(pool_id: b256, token: Address, amount: u64) -> u64 { 
    let pool_balances = storage.general_pools_balances.get(pool_id);
    let current_balance = _get_general_pool_balance_storage_map(pool_id, token);

    let token = ~ContractId::from(token.into());

    let new_balance = cash_to_managed(current_balance, amount);
    set(pool_id, (pool_id, token), new_balance);
    // pool_balances.insert(pool_balances, token, new_balance);

    return managed_delta(new_balance, current_balance);
}

#[storage(write, read)]
fn _update_general_pool_balance_managed_to_cash(pool_id: b256, token: Address, amount: u64) -> u64 { 
    let pool_balances = storage.general_pools_balances.get(pool_id);
    let current_balance = _get_general_pool_balance_storage_map(pool_id, token);

    let new_balance = managed_to_cash(current_balance, amount);
    let token = ~ContractId::from(token.into());
    set(pool_id, (pool_id, token), new_balance);

    return managed_delta(new_balance, current_balance);
}

#[storage(write, read)]
fn _update_general_pool_balance_set_managed(pool_id: b256, token: Address, amount: u64) -> u64 { 
    let pool_balances = storage.general_pools_balances.get(pool_id);
    let current_balance = _get_general_pool_balance_storage_map(pool_id, token);

    let new_balance = set_managed(current_balance, amount);
    let token = ~ContractId::from(token.into());
    set(pool_id, (pool_id, token), new_balance);

    return managed_delta(new_balance, current_balance);
}

// takes bytes; b256
#[storage(read)]
fn _get_general_pool_balance_bytes(pool_id: b256, token: Address) -> b256 {
    let pool_balances = storage.general_pools_balances.get(pool_id);

    return _get_general_pool_balance_storage_map(pool_id, token);
}

// // takes storagemap; StorageMap
#[storage(read)]
fn _get_general_pool_balance_storage_map(pool_id: b256, token: Address) -> b256 {

    return get(pool_id, ~ContractId::from(token.into()));
}


