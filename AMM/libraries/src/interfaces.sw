library interface;

dep data_structures;

use data_structures::{PoolInfo, PreviewAddLiquidityInfo, PreviewSwapInfo, RemoveLiquidityInfo};
use std::contract_id::ContractId;

abi AMM {
    /// Initialize the AMM by setting the valid exchange contract bytecode root.
    ///
    /// # Arguments
    /// 
    /// - ` exchange_contract_id` - factory exchange contract identifier
    ///
    /// # Reverts 
    ///
    /// * When the AMM has already been initialized
    #[storage(read, write)]
    fn initialize(exchange_contract_id: ContractId);
    /// Add an (asset pair, exchange contract ID) mapping to the storage.
    /// 
    /// # Arguments
    /// 
    /// - ` asset_pair ` - tuple of identifiers of the pair of assets that make up the pool
    /// - ` pool ` - identifier of exchange contract that defines the pool of the given pair
    /// 
    /// # Reverts
    /// 
    /// * When the AMM contract has not been initialized
    /// * When the bytecode root of ` pool ` does not match the bytecode root of the factory exchange contract
    /// * When the pool info of the exchange contract with the given address does not consist of the given asset pair
    #[storage(read, write)]
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
    /// Mint liquidity pool asset at current ratio (that is calculated from deposit amounts in the contract)
    /// and transfer to the sender. Move previously deposited assets of correspoding amounts to contract reserves.
    /// 
    /// # Arguments
    /// 
    /// - ` deadline ` - limit on block height for operation
    /// - ` min_liquidity ` - minimum amount of liquidity to add
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    /// * When the current block height is not less than `deadline`
    /// * When the ` msg_amount ` with function call is not 0
    /// * When the ` msg_asset_id ` does not identify asset A or asset B
    /// * When the sender's asset A deposit amount is 0
    /// * When the contract already has some liquidity and ` min_liquidity ` is 0
    /// * When the contract already has some liquidity and the liquidity to mint is less than ` min_liquidity `
    /// * When the contract has no liquidity and the sender's asset A deposit amount is less than ` minimum_liquidity ` (that is a config-time parameter)
    #[storage(read, write)]
    fn add_liquidity(deadline: u64, min_liquidity: u64) -> u64;
    /// Initialize contract by specifying the asset pair that makes up the pool.
    /// 
    /// # Arguments
    /// 
    /// - ` pair ` - unique identifiers of the asset pair, i.e., asset A and asset B
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    /// * When the passed pair describes identical assets
    #[storage(read, write)]
    fn constructor(pair: (ContractId, ContractId));
    /// Deposit asset to later add to the liquidity pool or withdraw.
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    /// * When the ` msg_asset_id ` does not identify asset A or asset B
    #[storage(read, write)]
    fn deposit();
    /// Get the preview info of a ` swap_with_maximum `, that consists of the maximum amount of
    /// input asset to forward to get ` exact_output_amount ` amount of other asset and
    /// whether the input asset reserves are sufficient for the swap or not.
    /// 
    /// # Arguments
    /// 
    /// - ` exact_output_amount ` - the desired amount of other asset to receive after swap
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    /// * When the ` msg_asset_id ` does not identify asset A or asset B
    /// * When the ` amount ` is less than the reserve amount of the other asset
    #[storage(read, write)]
    fn preview_swap_with_maximum(exact_output_amount: u64) -> PreviewSwapInfo;
    /// Get the preview info of a ` swap_with_minimum `, that consists of the amount of
    /// output asset to receive by forwarding ` exact_input_amount` of input asset and whether the
    /// output asset reserves are sufficient for the swap or not.
    /// 
    /// # Arguments
    /// 
    /// - ` exact_input_amount ` - the amount of other asset to swap
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    /// * When the ` msg_asset_id ` does not identify asset A or asset B
    #[storage(read, write)]
    fn preview_swap_with_minimum(exact_input_amount: u64) -> PreviewSwapInfo;
    /// Burn liquidity pool asset at current ratio and transfer asset A and asset B to the sender.
    /// 
    /// # Arguments
    /// 
    /// - ` deadline ` - limit on block height for operation
    /// - ` min_asset_a ` - minimum amount of asset A to receive after burn
    /// - ` min_asset_b ` - minimum amount of asset B to receive after burn
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    /// * When there are no liquidity pool assets to burn
    /// * When the ` msg_asset_id ` does not identify the liquidity pool asset
    /// * When ` min_asset_a ` or ` min_asset_b ` is 0
    /// * When the current block height is not less than ` deadline `
    /// * When the ` msg_amount ` with function call is 0
    /// * When the minimum amounts for asset A and asset B to receive after burn cannot be satisfied
    #[storage(read, write)]
    fn remove_liquidity(deadline: u64, min_asset_a: u64, min_asset_b: u64) -> RemoveLiquidityInfo;
    /// Swap the forwarded amount of forwarded asset for ` exact_output_amount ` of other asset and
    /// transfer any extra forwarded amount of forwarded asset and output asset to sender.
    /// 
    /// # Arguments
    /// 
    /// - ` deadline ` - limit on block height for operation
    /// - ` exact_output_amount ` - the exact amount of output asset to receive
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    /// * When the ` msg_asset_id ` does not identify asset A or asset B
    /// * When ` exact_output_amount ` is 0
    /// * When the current block height is not less than ` deadline `
    /// * When the ` msg_amount ` with function call is 0
    /// * When the forwarded amount is insufficient for swap
    #[storage(read, write)]
    fn swap_with_maximum(deadline: u64, exact_output_amount: u64) -> u64;
    /// Swap ` exact_input_amount ` of forwarded asset for minimum forwarded amount of other asset
    /// and transfer output asset to sender.
    /// 
    /// # Arguments
    /// 
    /// - ` deadline ` - limit on block height for operation
    /// - ` exact_input_amount ` - the exact amount of input asset to sell
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    /// * When the ` msg_asset_id ` does not identify asset A or asset B
    /// * When ` min ` is 0
    /// * When the current block height is not less than `deadline`
    /// * When the ` msg_amount ` with function call is 0
    /// * When the forwarded amount is excessive for swap
    #[storage(read, write)]
    fn swap_with_minimum(deadline: u64, exact_input_amount: u64) -> u64;
    /// Withdraw coins that have not been added to a liquidity pool yet.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - the amount of coins to withdraw
    /// - ` asset ` - identifier of the asset to withdraw
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    /// * When the ` msg_asset_id ` does not identify asset A or asset B
    /// * When the deposited amount by the sender stored in the contract is insufficient
    #[storage(read, write)]
    fn withdraw(amount: u64, asset: ContractId);
    /// Get current balance of the sender for a given asset on the contract.
    /// 
    /// # Arguments
    /// 
    /// - ` asset ` - identifier of the asset to get balance of
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    #[storage(read)]
    fn balance(asset: ContractId) -> u64;
    /// Get the pool info of the exchange contract, i.e., asset A and B identifiers, asset A and B amounts
    /// and liquidity pool asset supply amount.
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    #[storage(read)]
    fn pool_info() -> PoolInfo;
    /// Get the preview info of adding liquidity, that consists of the he amount of other asset
    /// to input given the current ratio and the amount of liquidity pool asset that will be received.
    /// 
    /// # Arguments
    /// 
    /// - ` amount ` - amount of an asset to add
    /// - ` asset ` - identifier of the asset to add
    /// 
    /// # Reverts
    /// 
    /// * When the contract has not been initialized, i.e., asset pair in storage is ` None `
    /// * When the ` msg_asset_id ` does not identify asset A or asset B
    /// * When the contract has no liquidity and the function is called for asset B
    #[storage(read)]
    fn preview_add_liquidity(amount: u64, asset: ContractId) -> PreviewAddLiquidityInfo;
}
