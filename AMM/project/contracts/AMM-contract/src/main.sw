contract;

mod errors;
mod events;

use ::errors::InitError;
use ::events::{RegisterPoolEvent, SetExchangeBytecodeRootEvent};
use libraries::{AMM, Exchange};
use std::{constants::BASE_ASSET_ID, external::bytecode_root};

storage {
    /// The valid exchange contract bytecode root.
    exchange_bytecode_root: Option<b256> = Option::None,
    /// Map that stores pools, i.e., asset identifier pairs as keys and corresponding exchange contract identifiers as values.
    pools: StorageMap<(ContractId, ContractId), ContractId> = StorageMap {},
}

impl AMM for Contract {
    /// Initialize the AMM by specifying the exchange contract bytecode root, for security.
    ///
    /// ### Arguments
    ///
    /// * `exchange_bytecode_root`: `ContractId` - The bytecode root of the intended implementation of the exchange ABI.
    ///
    /// # Reverts
    ///
    /// * When the AMM has already been initialized.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    #[storage(read, write)]
    fn initialize(exchange_bytecode_root: ContractId) {
        require(storage.exchange_bytecode_root.read().is_none(), InitError::BytecodeRootAlreadySet);
        storage.exchange_bytecode_root.write(Option::Some(exchange_bytecode_root.into()));
        log(SetExchangeBytecodeRootEvent {
            root: exchange_bytecode_root.into(),
        });
    }

    /// Add an (asset pair, exchange contract ID) mapping to the storage.
    ///
    /// ### Arguments
    ///
    /// * `asset_pair`: `(ContractId, ContractId)` - The pair of assets that make up the pool.
    /// * `pool`: `ContractId` - The pair of assets that make up the pool.
    ///
    /// # Reverts
    ///
    /// * When the AMM contract has not been initialized
    /// * When the bytecode root of `pool` does not match the bytecode root of the intended exchange contract
    /// * When the pool info of the exchange contract with the given address does not consist of the given asset pair
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `2`
    /// * Writes: `1`
    #[storage(read, write)]
    fn add_pool(asset_pair: (ContractId, ContractId), pool: ContractId) {
        require(storage.exchange_bytecode_root.read().is_some(), InitError::BytecodeRootNotSet);
        require(storage.exchange_bytecode_root.read().unwrap() == bytecode_root(pool), InitError::BytecodeRootDoesNotMatch);

        let exchange_contract = abi(Exchange, pool.into());
        let pool_info = exchange_contract.pool_info();
        let pair = pool_info.reserves;
        let pair_matches_exchange_pair = (pair.a.id == asset_pair.0 && pair.b.id == asset_pair.1) || (pair.a.id == asset_pair.1 && pair.b.id == asset_pair.0);

        require(pair_matches_exchange_pair, InitError::PairDoesNotDefinePool);

        let ordered_asset_pair = if asset_pair.0.into() < asset_pair.1.into() {
            asset_pair
        } else {
            (asset_pair.1, asset_pair.0)
        };

        storage.pools.insert(ordered_asset_pair, pool);

        log(RegisterPoolEvent {
            asset_pair: ordered_asset_pair,
            pool,
        });
    }

    /// For the given asset pair, get the exchange contract, i.e., the pool that consists of the asset pair.
    ///
    /// # Arguments
    ///
    /// - `asset_pair` - pair of assets that make up the pool

    /// For the given asset pair, get the exchange contract; the pool that consists of the asset pair.
    ///
    /// ### Arguments
    ///
    /// * `asset_pair`: `(ContractId, ContractId)` - The pair of assets that make up the pool.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `1`
    #[storage(read)]
    fn pool(asset_pair: (ContractId, ContractId)) -> Option<ContractId> {
        let ordered_asset_pair = if asset_pair.0.into() < asset_pair.1.into() {
            asset_pair
        } else {
            (asset_pair.1, asset_pair.0)
        };
        storage.pools.get(ordered_asset_pair).try_read()
    }
}
