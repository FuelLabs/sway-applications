contract;

mod errors;
mod events;

use ::errors::InitError;
use ::events::{RegisterPoolEvent, SetExchangeBytecodeRootEvent};
use libraries::{AMM, data_structures::PoolInfo, Exchange};
use std::{external::bytecode_root, hash::Hash};

storage {
    /// The valid exchange contract bytecode root.
    exchange_bytecode_root: Option<b256> = Option::None,
    /// Map that stores pools, i.e., asset identifier pairs as keys and corresponding exchange contract identifiers as values.
    pools: StorageMap<(AssetId, AssetId), ContractId> = StorageMap {},
}

impl AMM for Contract {
    #[storage(read, write)]
    fn initialize(exchange_bytecode_root: ContractId) {
        require(
            storage
                .exchange_bytecode_root
                .read()
                .is_none(),
            InitError::BytecodeRootAlreadySet,
        );
        storage
            .exchange_bytecode_root
            .write(Option::Some(exchange_bytecode_root.into()));
        log(SetExchangeBytecodeRootEvent {
            root: exchange_bytecode_root.into(),
        });
    }

    #[storage(read, write)]
    fn add_pool(asset_pair: (AssetId, AssetId), pool: ContractId) {
        require(
            storage
                .exchange_bytecode_root
                .read()
                .is_some(),
            InitError::BytecodeRootNotSet,
        );
        require(
            storage
                .exchange_bytecode_root
                .read()
                .unwrap() == bytecode_root(pool),
            InitError::BytecodeRootDoesNotMatch,
        );

        let exchange_contract = abi(Exchange, pool.into());
        let pool_info = exchange_contract.pool_info();
        let pair = pool_info.reserves;
        let pair_matches_exchange_pair = (pair.a.id == asset_pair.0 && pair.b.id == asset_pair.1) || (pair.a.id == asset_pair.1 && pair.b.id == asset_pair.0);

        require(pair_matches_exchange_pair, InitError::PairDoesNotDefinePool);

        let asset_pair_0_b256: b256 = asset_pair.0.into();
        let asset_pair_1_b256: b256 = asset_pair.1.into();

        let ordered_asset_pair = if asset_pair_0_b256 < asset_pair_1_b256 {
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

    #[storage(read)]
    fn pool(asset_pair: (AssetId, AssetId)) -> Option<ContractId> {
        let asset_pair_0_b256: b256 = asset_pair.0.into();
        let asset_pair_1_b256: b256 = asset_pair.1.into();
        let ordered_asset_pair = if asset_pair_0_b256 < asset_pair_1_b256 {
            asset_pair
        } else {
            (asset_pair.1, asset_pair.0)
        };
        storage.pools.get(ordered_asset_pair).try_read()
    }
}
