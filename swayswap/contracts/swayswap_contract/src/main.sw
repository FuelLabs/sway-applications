contract;

use std::{
    address::*,
    block::*,
    context::{
        *,
        call_frames::*,
    },
    contract_id::ContractId,
    identity::Identity,
    revert::revert,
    storage::StorageMap,
};

use swayswap_abi::SwaySwap;

/// Store token ID and exchange contract ID in storage
storage {
    tokens: StorageMap<b256, b256> = StorageMap {},
}

impl SwaySwap for Contract {
    #[storage(write)]
    fn add_exchange_contract(token_id: ContractId, exchange_id: ContractId) {
        storage.tokens.insert(token_id.into(), exchange_id.into());
    }
    #[storage(read)]
    fn get_exchange_contract(token_id: ContractId) -> ContractId {
        ~ContractId::from(storage.tokens.get(token_id.into()))
    }
}
