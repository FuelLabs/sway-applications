contract;

dep errors;
dep events;
dep utils;

use errors::InitError;
use events::{RegisterPoolEvent, SetExchangeBytecodeRootEvent};
use libraries::{AMM, Exchange};
use std::{constants::BASE_ASSET_ID, logging::log, storage::StorageMap};
use utils::bytecode_root;

storage {
    /// The valid exchange contract bytecode root
    exchange_bytecode_root: Option<b256> = Option::None,
    /// Map that stores pools, i.e., asset identifier pairs as keys and corresponding exchange contract identifiers as values
    pools: StorageMap<(ContractId, ContractId), ContractId> = StorageMap {},
}

impl AMM for Contract {
    #[storage(read, write)]
    fn initialize(exchange_id: ContractId) {
        require(storage.exchange_bytecode_root.is_none(), InitError::BytecodeRootAlreadySet);
        let root = bytecode_root(exchange_id);
        storage.exchange_bytecode_root = Option::Some(root);
        log(SetExchangeBytecodeRootEvent { root });
    }

    #[storage(read, write)]
    fn add_pool(asset_pair: (ContractId, ContractId), pool: ContractId) {
        require(storage.exchange_bytecode_root.is_some(), InitError::BytecodeRootNotSet);
        require(storage.exchange_bytecode_root.unwrap() == bytecode_root(pool), InitError::BytecodeRootDoesNotMatch);

        let exchange_contract = abi(Exchange, pool.into());
        let pool_info = exchange_contract.pool_info();
        let pair_matches_exchange_pair = (pool_info.asset_a == asset_pair.0 && pool_info.asset_b == asset_pair.1) || (pool_info.asset_a == asset_pair.1 && pool_info.asset_b == asset_pair.0);

        require(pair_matches_exchange_pair, InitError::PairDoesNotDefinePool);

        let ordered_asset_pair = if asset_pair.0.into() < asset_pair.1.into() {
            asset_pair
        } else {
            (asset_pair.1, asset_pair.0)
        };
        storage.pools.insert(ordered_asset_pair, pool);
        log(RegisterPoolEvent {
            pair: ordered_asset_pair,
            pool,
        });
    }

    #[storage(read)]
    fn pool(asset_pair: (ContractId, ContractId)) -> Option<ContractId> {
        let ordered_asset_pair = if asset_pair.0.into() < asset_pair.1.into() {
            asset_pair
        } else {
            (asset_pair.1, asset_pair.0)
        };
        let pool_id = storage.pools.get(ordered_asset_pair);
        if pool_id == BASE_ASSET_ID {
            Option::None
        } else {
            Option::Some(pool_id)
        }
    }
}
