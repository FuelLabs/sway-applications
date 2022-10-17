contract;

dep errors;

use errors::InitError;
use libraries::{AMM, Exchange};
use std::{constants::BASE_ASSET_ID, contract_id::ContractId, storage::StorageMap};

storage {
    /// Map that stores pools, i.e., asset identifier pairs as keys and corresponding exchange contract identifiers as values
    pools: StorageMap<(ContractId, ContractId), ContractId> = StorageMap {},
}

impl AMM for Contract {
    #[storage(write)]
    fn add_pool(asset_pair: (ContractId, ContractId), pool: ContractId) {
        let exchange_contract = abi(Exchange, pool.into());
        let pool_info = exchange_contract.pool_info();
        require(pool_info.asset_a_id == asset_pair.0 && pool_info.asset_b_id == asset_pair.1, InitError::ExchangeContractDoesNotMatchPair);
        storage.pools.insert(asset_pair, pool);
    }
    #[storage(read)]
    fn pool(asset_pair: (ContractId, ContractId)) -> Option<ContractId> {
        let pool_id = storage.pools.get(asset_pair);
        if pool_id == BASE_ASSET_ID {
            Option::None
        } else {
            Option::Some(pool_id)
        }
    }
}
