contract;

dep interface;

use interface::AMM;
use std::{contract_id::ContractId, storage::StorageMap};

storage {
    /// Map that stores (asset ID, exchange contract ID) pairs
    pools: StorageMap<ContractId, ContractId> = StorageMap {},
}

impl AMM for Contract {
    #[storage(write)]
    fn add_exchange_contract_to_asset(asset_id: ContractId, exchange_id: ContractId) {
        storage.pools.insert(asset_id, exchange_id);
    }
    #[storage(read)]
    fn exchange_contract_of_asset(asset_id: ContractId) -> ContractId {
        storage.pools.get(asset_id)
    }
}
