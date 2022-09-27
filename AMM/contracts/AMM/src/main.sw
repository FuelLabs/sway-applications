contract;

dep interface;

use interface::AMM;
use std::{contract_id::ContractId, storage::StorageMap};

storage {
    /// Map that stores (exchange contract ID, token contract ID)
    tokens: StorageMap<ContractId, ContractId> = StorageMap {},
}

impl AMM for Contract {
    #[storage(write)]
    fn add_exchange_contract(exchange_id: ContractId, token_id: ContractId) {
        storage.tokens.insert(token_id, exchange_id);
    }
    #[storage(read)]
    fn get_exchange_contract(token_id: ContractId) -> ContractId {
        storage.tokens.get(token_id)
    }
}
