library interface;

dep data_structures;

use data_structures::{PoolInfo, PreviewAddLiquidityInfo, PreviewSwapInfo, RemoveLiquidityInfo};
use std::contract_id::ContractId;

abi AMM {
    /// Add an (asset pair, exchange contract ID) mapping to the storage.
    /// 
    /// # Arguments
    /// 
    /// - ` asset_pair ` - tuple of identifiers of the pair of assets that make up the pool
    /// - ` pool ` - identifier of exchange contract that defines the pool of the given pair
    /// 
    /// # Reverts
    /// 
    /// * When the pool info of the exchange contract with the given address does not consist of the given asset pair
    #[storage(write)]
    fn add_pool(asset_pair: (ContractId, ContractId), pool: ContractId);
    /// For the given asset pair, get the exchange contract, i.e., the pool that consists of the asset pair.
    /// 
    /// # Arguments
    /// 
    /// - ` asset_pair ` - tuple of identifiers of the pair of assets that make up the pool
    #[storage(read)]
    fn pool(asset_pair: (ContractId, ContractId)) -> Option<ContractId>;
}

abi Exchange {
    /// Mint base asset and other asset at current ratio and add to liquidity pool.
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
    /// * When the associated ` msg_asset_id ` is not of either base asset or the other asset
    /// * If the sender does not have base asset balance in contract
    /// * When total liquidity exists but ` min_liquidity ` is 0
    /// * When the calculated mint amount is lesser than ` min_liquidity `
    /// * In the case of liquidity pool being empty, when sender base asset balance is lesser than ` MINIMUM_LIQUIDITY `
    #[storage(read, write)]
    fn add_liquidity(deadline: u64, min_liquidity: u64) -> u64;

    /// Get current balance of given asset on the contract.
    /// 
    /// # Arguments
    /// 
    /// - ` asset ` - identifier of the asset to get balance of
    #[storage(read)]
    fn balance(asset: ContractId) -> u64;

    /// Initialize contract by specifying the asset on the other side of the contract.
    /// 
    /// # Arguments
    /// 
    /// - ` asset ` - identifier of other asset
    #[storage(read, write)]
    fn constructor(asset: ContractId);

    /// Deposit coins for later adding to the liquidity pool.
    /// 
    /// # Reverts
    /// 
    /// * When the ` msg_asset_id ` does not identify either one of the assets in the pool
    #[storage(read, write)]
    fn deposit();

    /// Get information on the liquidity pool on contract.
    #[storage(read)]
    fn pool_info() -> PoolInfo;

    /// Preview "add liquidity" information.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - amount of liquidity to add
    /// - ` asset ` - identifier of the asset to add
    #[storage(read)]
    fn preview_add_liquidity(amount: u64, asset: ContractId) -> PreviewAddLiquidityInfo;

    /// Get required amount of coins for a ` swap_with_maximum `.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - amount of assets supplied
    /// 
    /// # Reverts
    /// 
    /// * When the reserve of the asset with provided ` msg_asset_id ` is insufficient
    #[storage(read, write)]
    fn preview_swap_with_maximum(amount: u64) -> PreviewSwapInfo;

    /// Get the minimum amount of coins that will be received for a ` swap_with_minimum `.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - amount of assets supplied
    #[storage(read, write)]
    fn preview_swap_with_minimum(amount: u64) -> PreviewSwapInfo;

    /// Burn assets to transfer base asset and other asset at current ratio to the sender.
    /// 
    /// # Arguments
    /// 
    /// - ` deadline ` - limit on block height for operation
    /// - ` min_base_asset ` - minimum amount of base asset to calculate amount to transfer
    /// - ` min_other_asset ` - minimum amount of other asset to calculate amount to transfer
    /// 
    /// # Reverts
    /// 
    /// * When the associated ` msg_amount ` with function call is 0
    /// * When the associated ` msg_asset_id ` does not match ` contract_id `
    /// * When the deadline has passed, i.e.: deadline is greater than current block height
    /// * When the associated ` min_base_asset ` is 0
    /// * When the associated ` min_other_asset ` is 0
    /// * When the total liquidity in the pool is 0
    /// * When the calculated base asset amount to transfer is lesser than ` min_base_asset `
    /// * When the calculated other asset amount to transfer is lesser than ` min_other_asset `
    #[storage(read, write)]
    fn remove_liquidity(deadline: u64, min_base_asset: u64, min_other_asset: u64) -> RemoveLiquidityInfo;

    /// Swap base asset <-> other asset and transfer to the sender.
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
    /// * When the associated ` msg_asset_id ` is not of either base asset or the other asset
    /// * When the passed ` amount ` is insufficient for swap
    #[storage(read, write)]
    fn swap_with_maximum(amount: u64, deadline: u64) -> u64;

    /// Swap base asset <-> other asset and transfer to the sender.
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
    /// * When the associated ` msg_asset_id ` is not of either base asset or the other asset
    /// * When the resulting amount is lesser than the provided minimum
    #[storage(read, write)]
    fn swap_with_minimum(deadline: u64, min: u64) -> u64;

    /// Withdraw coins that have not been added to a liquidity pool yet.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - amount of coins to withdraw
    /// - ` asset ` - identifier of asset to withdraw
    /// 
    /// # Reverts
    /// 
    /// * When the ` asset ` is not of either base asset or the other asset
    /// * If the sender does not have ` amount ` of asset in contract storage
    #[storage(read, write)]
    fn withdraw(amount: u64, asset: ContractId);
}
