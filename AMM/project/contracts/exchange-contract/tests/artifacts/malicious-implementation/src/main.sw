contract;

use libraries::{
    data_structures::{
        Asset,
        AssetPair,
        PoolInfo,
        PreviewAddLiquidityInfo,
        PreviewSwapInfo,
        RemoveLiquidityInfo,
    },
    Exchange,
};
use std::{call_frames::contract_id, constants::BASE_ASSET_ID};

storage {
    pair: Option<AssetPair> = Option::None,
}

impl Exchange for Contract {
    #[storage(read, write)]
    fn add_liquidity(desired_liquidity: u64, deadline: u64) -> u64 {
        0
    }

    #[storage(read, write)]
    fn constructor(asset_a: ContractId, asset_b: ContractId) {
        storage.pair = Option::Some(AssetPair::new(Asset::new(asset_a, 0), Asset::new(asset_b, 0)));
    }

    #[payable, storage(read, write)]
    fn deposit() {}

    #[payable, storage(read, write)]
    fn remove_liquidity(min_asset_a: u64, min_asset_b: u64, deadline: u64) -> RemoveLiquidityInfo {
        RemoveLiquidityInfo {
            removed_amounts: storage.pair.unwrap(),
            burned_liquidity: Asset::new(contract_id(), 0),
        }
    }

    #[payable, storage(read, write)]
    fn swap_exact_input(min_output: Option<u64>, deadline: u64) -> u64 {
        0
    }

    #[payable, storage(read, write)]
    fn swap_exact_output(output: u64, deadline: u64) -> u64 {
        0
    }

    #[storage(read, write)]
    fn withdraw(asset: Asset) {}

    #[storage(read)]
    fn balance(asset_id: ContractId) -> u64 {
        0
    }

    #[storage(read)]
    fn pool_info() -> PoolInfo {
        PoolInfo {
            reserves: storage.pair.unwrap_or(AssetPair::new(Asset::new(BASE_ASSET_ID, 0), Asset::new(BASE_ASSET_ID, 0))),
            liquidity: 0,
        }
    }

    #[storage(read)]
    fn preview_add_liquidity(asset: Asset) -> PreviewAddLiquidityInfo {
        PreviewAddLiquidityInfo {
            other_asset_to_add: storage.pair.unwrap().other_asset(asset.id),
            liquidity_asset_to_receive: Asset::new(contract_id(), 0),
        }
    }

    #[storage(read)]
    fn preview_swap_exact_input(exact_input_asset: Asset) -> PreviewSwapInfo {
        PreviewSwapInfo {
            other_asset: storage.pair.unwrap().other_asset(exact_input_asset.id),
            sufficient_reserve: false,
        }
    }

    #[storage(read)]
    fn preview_swap_exact_output(exact_output_asset: Asset) -> PreviewSwapInfo {
        PreviewSwapInfo {
            other_asset: storage.pair.unwrap().other_asset(exact_output_asset.id),
            sufficient_reserve: false,
        }
    }
}
