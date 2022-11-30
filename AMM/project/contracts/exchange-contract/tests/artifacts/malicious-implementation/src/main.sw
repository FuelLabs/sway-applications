contract;

use libraries::{
    data_structures::{
        PoolInfo,
        PreviewAddLiquidityInfo,
        PreviewSwapInfo,
        RemoveLiquidityInfo,
    },
    Exchange,
};
use std::constants::BASE_ASSET_ID;

storage {
    pair: Option<(ContractId, ContractId)> = Option::None,
}

impl Exchange for Contract {
    #[storage(read, write)]
    fn add_liquidity(
        desired_liquidity: u64,
        deadline: u64,
        recipient: Option<Identity>,
    ) -> u64 {
        0
    }

    #[storage(read, write)]
    fn constructor(pair: (ContractId, ContractId)) {
        storage.pair = Option::Some(pair);
    }

    #[storage(read, write)]
    fn deposit() {}

    #[storage(read, write)]
    fn remove_liquidity(min_asset_a: u64, min_asset_b: u64, deadline: u64) -> RemoveLiquidityInfo {
        RemoveLiquidityInfo {
            asset_a_amount: 0,
            asset_b_amount: 0,
            liquidity: 0,
        }
    }

    #[storage(read, write)]
    fn swap_with_exact_input(min_output: Option<u64>, deadline: u64) -> u64 {
        0
    }

    #[storage(read, write)]
    fn swap_with_exact_output(output: u64, deadline: u64) -> u64 {
        0
    }

    #[storage(read, write)]
    fn withdraw(amount: u64, asset: ContractId) {}

    #[storage(read)]
    fn balance(asset: ContractId) -> u64 {
        0
    }

    #[storage(read)]
    fn pool_info() -> PoolInfo {
        let pair = if storage.pair.is_some() {
            storage.pair.unwrap()
        } else {
            (BASE_ASSET_ID, BASE_ASSET_ID)
        };
        PoolInfo {
            asset_a: pair.0,
            asset_b: pair.1,
            asset_a_reserve: 0,
            asset_b_reserve: 0,
            liquidity: 0,
        }
    }

    #[storage(read)]
    fn preview_add_liquidity(amount: u64, asset: ContractId) -> PreviewAddLiquidityInfo {
        PreviewAddLiquidityInfo {
            other_asset_amount_to_add: 0,
            liquidity_asset_amount_to_receive: 0,
        }
    }

    #[storage(read)]
    fn preview_swap_with_exact_input(exact_input: u64, input_asset: ContractId) -> PreviewSwapInfo {
        PreviewSwapInfo {
            amount: 0,
            sufficient_reserve: false,
        }
    }

    #[storage(read)]
    fn preview_swap_with_exact_output(exact_output: u64, input_asset: ContractId) -> PreviewSwapInfo {
        PreviewSwapInfo {
            amount: 0,
            sufficient_reserve: false,
        }
    }
}
