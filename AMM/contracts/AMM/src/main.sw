contract;

dep interface;

use interface::AMM;
use std::{contract_id::ContractId, storage::StorageMap};

/// Store token ID and exchange contract ID in storage
storage {
    tokens: StorageMap<b256, b256> = StorageMap {},
}

impl AMM for Contract {
    #[storage(write)]
    fn add_exchange_contract(exchange_id: ContractId, token_id: ContractId) {
        storage.tokens.insert(token_id.into(), exchange_id.into());
    }
    #[storage(read)]
    fn get_exchange_contract(token_id: ContractId) -> ContractId {
        ~ContractId::from(storage.tokens.get(token_id.into()))
    }
}
