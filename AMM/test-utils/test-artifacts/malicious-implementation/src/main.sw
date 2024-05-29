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

storage {
    pair: Option<AssetPair> = Option::None,
}

impl Exchange for Contract {
    #[storage(read, write)]
    fn add_liquidity(desired_liquidity: u64, deadline: u64) -> u64 {
        0
    }

    #[storage(read, write)]
    fn constructor(asset_a: AssetId, asset_b: AssetId) {
        storage
            .pair
            .write(Option::Some(AssetPair::new(Asset::new(asset_a, 0), Asset::new(asset_b, 0))));
    }

    #[payable, storage(read, write)]
    fn deposit() {}

    #[payable, storage(read, write)]
    fn remove_liquidity(min_asset_a: u64, min_asset_b: u64, deadline: u64) -> RemoveLiquidityInfo {
        RemoveLiquidityInfo {
            removed_amounts: storage.pair.read().unwrap(),
            burned_liquidity: Asset::new(AssetId::default(), 0),
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
    fn balance(asset_id: AssetId) -> u64 {
        0
    }

    #[storage(read)]
    fn pool_info() -> PoolInfo {
        PoolInfo {
            reserves: storage.pair.read().unwrap_or(AssetPair::new(
                Asset::new(AssetId::base(), 0),
                Asset::new(AssetId::base(), 0),
            )),
            liquidity: 0,
        }
    }

    #[storage(read)]
    fn preview_add_liquidity(asset: Asset) -> PreviewAddLiquidityInfo {
        PreviewAddLiquidityInfo {
            other_asset_to_add: storage.pair.read().unwrap().other_asset(asset.id),
            liquidity_asset_to_receive: Asset::new(AssetId::default(), 0),
        }
    }

    #[storage(read)]
    fn preview_swap_exact_input(exact_input_asset: Asset) -> PreviewSwapInfo {
        PreviewSwapInfo {
            other_asset: storage.pair.read().unwrap().other_asset(exact_input_asset.id),
            sufficient_reserve: false,
        }
    }

    #[storage(read)]
    fn preview_swap_exact_output(exact_output_asset: Asset) -> PreviewSwapInfo {
        PreviewSwapInfo {
            other_asset: storage.pair.read().unwrap().other_asset(exact_output_asset.id),
            sufficient_reserve: false,
        }
    }
}
