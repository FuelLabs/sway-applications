library;

mod data_structures;

use ::data_structures::{
    Asset,
    PoolInfo,
    PreviewAddLiquidityInfo,
    PreviewSwapInfo,
    RemoveLiquidityInfo,
};

abi AMM {
    #[storage(read, write)]
    fn initialize(exchange_bytecode_root: ContractId);

    #[storage(read, write)]
    fn add_pool(asset_pair: (ContractId, ContractId), pool: ContractId);

    #[storage(read)]
    fn pool(asset_pair: (ContractId, ContractId)) -> Option<ContractId>;
}

abi Exchange {
    #[storage(read, write)]
    fn add_liquidity(desired_liquidity: u64, deadline: u64) -> u64;

    #[storage(read, write)]
    fn constructor(asset_a: ContractId, asset_b: ContractId);

    #[payable, storage(read, write)]
    fn deposit();

    #[payable, storage(read, write)]
    fn remove_liquidity(min_asset_a: u64, min_asset_b: u64, deadline: u64) -> RemoveLiquidityInfo;

    #[payable, storage(read, write)]
    fn swap_exact_input(min_output: Option<u64>, deadline: u64) -> u64;

    #[payable, storage(read, write)]
    fn swap_exact_output(output: u64, deadline: u64) -> u64;

    #[storage(read, write)]
    fn withdraw(asset: Asset);

    #[storage(read)]
    fn balance(asset_id: ContractId) -> u64;

    #[storage(read)]
    fn pool_info() -> PoolInfo;

    #[storage(read)]
    fn preview_add_liquidity(asset: Asset) -> PreviewAddLiquidityInfo;

    #[storage(read)]
    fn preview_swap_exact_input(exact_input_asset: Asset) -> PreviewSwapInfo;

    #[storage(read)]
    fn preview_swap_exact_output(exact_output_asset: Asset) -> PreviewSwapInfo;
}
