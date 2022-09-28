library interface;

dep data_structures;

use data_structures::{PoolInfo, PreviewAddLiquidityInfo, PreviewInfo, RemoveLiquidityInfo};
use std::contract_id::ContractId;

abi Exchange {
    /// Mint ETH and Token at current ratio and add to liquidity pool.
    /// 
    /// # Arguments
    /// 
    /// - ` deadline ` - limit on block height for operation
    /// - ` min_liquidity ` - minimum amount of liquidity to add
    /// 
    /// # Reverts
    /// 
    /// * When the associated ` msg_amount ` with function call is not 0
    /// * When the deadline has passed, i.e.: deadline is greater than current block height
    /// * When the associated ` msg_asset_id ` is not of either ETH or Token
    /// * If the sender does not have ETH balance in contract
    /// * When total liquidity exists but ` min_liquidity ` is 0
    /// * When the calculated mint amount is lesser than ` min_liquidity `
    /// * In the case of liquidity pool being empty, when sender ETH balance is lesser than ` MINIMUM_LIQUIDITY `
    #[storage(read, write)]
    fn add_liquidity(deadline: u64, min_liquidity: u64) -> u64;

    /// Get current balance of given token on the contract.
    /// 
    /// # Arguments
    /// 
    /// - ` id ` - identifier of the asset to get balance of
    #[storage(read)]
    fn balance(id: ContractId) -> u64;

    /// Deposit coins for later adding to the liquidity pool.
    /// 
    /// # Reverts
    /// 
    /// * When the ` msg_asset_id ` does not identify either one of the tokens in the pool
    #[storage(read, write)]
    fn deposit();

    /// Initialize contract by specifying the asset on the other side of the contract.
    /// 
    /// # Arguments
    /// 
    /// - ` asset_id ` - identifier of other asset
    /// - ` asset_contract_id ` - contract identifier of other asset
    #[storage(write)]
    fn initialize(asset_id: ContractId, asset_contract_id: ContractId);

    /// Get information on the liquidity pool on contract.
    #[storage(read)]
    fn pool_info() -> PoolInfo;

    /// Preview "add liquidity" information.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - amount of liquidity to add
    /// - ` id ` - identifier of the asset to add
    #[storage(read)]
    fn preview_add_liquidity(amount: u64, id: ContractId) -> PreviewAddLiquidityInfo;

    /// Get required amount of coins for a ` swap_with_maximum `.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - amount of tokens supplied
    /// 
    /// # Reverts
    /// 
    /// * When the reserve of the token with provided ` msg_asset_id ` is insufficient
    #[storage(read, write)]
    fn preview_swap_with_maximum(amount: u64) -> PreviewInfo;

    /// Get the minimum amount of coins that will be received for a ` swap_with_minimum `.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - amount of tokens supplied
    #[storage(read, write)]
    fn preview_swap_with_minimum(amount: u64) -> PreviewInfo;

    /// Burn tokens to transfer ETH and Tokens at current ratio to the sender.
    /// 
    /// # Arguments
    /// 
    /// - ` deadline ` - limit on block height for operation
    /// - ` min_eth ` - minimum amount of ETH to calculate amount to transfer
    /// - ` min_tokens ` - minimum amount of Token to calculate amount to transfer
    /// 
    /// # Reverts
    /// 
    /// * When the associated ` msg_amount ` with function call is 0
    /// * When the associated ` msg_asset_id ` does not match ` contract_id `
    /// * When the deadline has passed, i.e.: deadline is greater than current block height
    /// * When the associated ` min_eth ` is 0
    /// * When the associated ` min_tokens ` is 0
    /// * When the total liquidity in the pool is 0
    /// * When the calculated ETH amount to transfer is lesser than ` min_eth `
    /// * When the calculated Token amount to transfer is lesser than ` min_tokens `
    #[storage(read, write)]
    fn remove_liquidity(deadline: u64, min_eth: u64, min_tokens: u64) -> RemoveLiquidityInfo;

    /// Swap ETH <-> Tokens and transfer to the sender.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - maximum amount of coins to swap
    /// - ` deadline ` - limit on block height for operation
    /// 
    /// # Reverts
    /// 
    /// * When the deadline has passed, i.e.: deadline is greater than current block height
    /// * When the passed ` amount ` is 0
    /// * When the associated ` msg_amount ` with function call is 0
    /// * When the associated ` msg_asset_id ` is not of either ETH or Token
    /// * When the passed ` amount ` is insufficient for swap
    #[storage(read, write)]
    fn swap_with_maximum(amount: u64, deadline: u64) -> u64;

    /// Swap ETH <-> Tokens and transfer to the sender.
    /// 
    /// # Arguments
    /// 
    /// - ` deadline ` - limit on block height for operation
    /// - ` min ` - minimum amount of coins of output
    /// 
    /// # Reverts
    /// 
    /// * When the deadline has passed, i.e.: deadline is greater than current block height
    /// * When the passed ` min ` is 0
    /// * When the associated ` msg_amount ` with function call is 0
    /// * When the associated ` msg_asset_id ` is not of either ETH or Token
    /// * When the resulting amount is lesser than the provided minimum
    #[storage(read, write)]
    fn swap_with_minimum(deadline: u64, min: u64) -> u64;

    /// Withdraw coins that have not been added to a liquidity pool yet.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - amount of coins to withdraw
    /// - ` id ` - identifier of asset to withdraw
    /// 
    /// # Reverts
    /// 
    /// * When the ` id ` is not of either ETH or Token
    /// * If the sender does not have ` amount ` of asset in contract storage
    #[storage(read, write)]
    fn withdraw(amount: u64, id: ContractId);
}
