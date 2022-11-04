contract;

dep data_structures;
dep interface;
dep utils;
dep errors;

use data_structures::PoolSpecialization;
use errors::Error;
use utils::{
    get_word_from_b256, 
    get_pool_address, 
    to_pool_id, 
    get_pool_specialization
};
use interface::PoolRegistry;

use std::{
    vec::Vec,
    contract_id::ContractId,
    option::Option,
    address::Address,
    storage::{StorageMap,get, store},
    token::{force_transfer_to_contract,transfer_to_output},
    chain::auth::{AuthError, msg_sender},
    revert::{revert, require},
    math::*,
    identity::Identity,
    result::*,
};

storage {
    is_pool_registered :StorageMap<b256, bool>= StorageMap {},
    next_pool_nonce: u64 = 0
}


impl PoolRegistry for Contract {
    #[storage(read)] 
    fn ensure_registered_pool(poolId: b256) {
        require(storage.is_pool_registered.get(poolId), Error::INVALID_POOL_ID);
    }

    #[storage(read,write)] 
    fn register_pool(specialization: PoolSpecialization ) ->b256
    {
        // Each Pool is assigned a unique ID based on an incrementing nonce. This assumes there will never be more than
        // 2//80 Pools, and the nonce will not overflow.
        let address = match msg_sender().unwrap() {
            Identity::Address(address) => address, _ => revert(0), 
        };
        let poolId: b256 = to_pool_id(address, specialization, (storage.next_pool_nonce));

        require(!(storage.is_pool_registered.get(poolId)), Error::INVALID_POOL_ID); // Should never happen as Pool IDs are unique.
        storage.is_pool_registered.insert(poolId, true);

        storage.next_pool_nonce = storage.next_pool_nonce + 1;

        return poolId;
    }
}


// Reverts unless `poolId` corresponds to a registered Pool.
#[storage(read)] 
fn ensure_registered_pool(poolId: b256) {
    require(storage.is_pool_registered.get(poolId), Error::INVALID_POOL_ID);
}


// Reverts unless `poolId` corresponds to a registered Pool, and the caller is the Pool's contract.
#[storage(read,write)]     
fn _ensure_pool_is_sender(poolId: b256) {
    ensure_registered_pool(poolId);
    let address = match msg_sender().unwrap() {
        Identity::Address(address) => address, _ => revert(0), 
    };
    require(address == get_pool_address(poolId), Error::CALLER_NOT_POOL);
}

#[storage(read)]
fn get_pool(poolId: b256)-> (Address, PoolSpecialization)
{
    return (get_pool_address(poolId), get_pool_specialization(poolId));
}

