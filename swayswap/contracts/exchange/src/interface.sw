library interface;

dep data_structures;

use data_structures::{
    PoolInfo,
    PositionInfo,
    PreviewAddLiquidityInfo,
    PreviewInfo,
    RemoveLiquidityInfo,
};
use std::contract_id::ContractId;

abi Exchange {
    /// Deposit ETH and Tokens at current ratio to mint SWAYSWAP tokens.
    #[storage(read, write)]
    fn add_liquidity(deadline: u64, min_liquidity: u64) -> u64;
    /// Deposit coins for later adding to liquidity pool.
    #[storage(read, write)]
    fn deposit();
    /// Get add liquidity preview
    #[storage(read)]
    fn get_add_liquidity(amount: u64, asset_id: b256) -> PreviewAddLiquidityInfo;
    /// Return the current balance of given token on the contract
    #[storage(read)]
    fn get_balance(asset_id: ContractId) -> u64;
    /// Get information on the liquidity pool
    #[storage(read)]
    fn get_pool_info() -> PoolInfo;
    /// Get current positions
    #[storage(read)]
    fn get_position(amount: u64) -> PositionInfo;
    /// Get required amount of coins for a swap_with_maximum.
    #[storage(read, write)]
    fn get_swap_with_maximum(amount: u64) -> PreviewInfo;
    /// Get the minimum amount of coins that will be received for a swap_with_minimum.
    #[storage(read, write)]
    fn get_swap_with_minimum(amount: u64) -> PreviewInfo;
    /// Burn SWAYSWAP tokens to withdraw ETH and Tokens at current ratio.
    #[storage(read, write)]
    fn remove_liquidity(deadline: u64, min_eth: u64, min_tokens: u64) -> RemoveLiquidityInfo;
    /// Swap ETH <-> Tokens and tranfers to sender.
    #[storage(read, write)]
    fn swap_with_maximum(amount: u64, deadline: u64) -> u64;
    /// Swap ETH <-> Tokens and tranfers to sender.
    #[storage(read, write)]
    fn swap_with_minimum(deadline: u64, min: u64) -> u64;
    /// Withdraw coins that have not been added to a liquidity pool yet.
    #[storage(read, write)]
    fn withdraw(amount: u64, asset_id: ContractId);
}
