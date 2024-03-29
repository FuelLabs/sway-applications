library;

pub mod data_structures;

use ::data_structures::{
    Asset,
    PoolInfo,
    PreviewAddLiquidityInfo,
    PreviewSwapInfo,
    RemoveLiquidityInfo,
};

abi AMM {
    /// Initialize the AMM by specifying the exchange contract bytecode root, for security.
    ///
    /// # Arguments
    ///
    /// * `exchange_bytecode_root`: [ContractId] - The bytecode root of the intended implementation of the exchange ABI.
    ///
    /// # Reverts
    ///
    /// * When the AMM has already been initialized.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    #[storage(read, write)]
    fn initialize(exchange_bytecode_root: ContractId);

    /// Add an (asset pair, exchange contract ID) mapping to the storage.
    ///
    /// # Arguments
    ///
    /// * `asset_pair`: [(AssetId, AssetId)] - The pair of assets that make up the pool.
    /// * `pool`: [ContractId] - The pair of assets that make up the pool.
    ///
    /// # Reverts
    ///
    /// * When the AMM contract has not been initialized
    /// * When the bytecode root of `pool` does not match the bytecode root of the intended exchange contract
    /// * When the pool info of the exchange contract with the given address does not consist of the given asset pair
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `2`
    /// * Writes: `1`
    #[storage(read, write)]
    fn add_pool(asset_pair: (AssetId, AssetId), pool: ContractId);

    /// For the given asset pair, get the exchange contract; the pool that consists of the asset pair.
    ///
    /// # Arguments
    ///
    /// * `asset_pair`: [(AssetId, AssetId)] - The pair of assets that make up the pool.
    ///
    /// # Returns
    ///
    /// * `pool`: [Option<ContractId>] - The exchange contract that consists of the given asset pair.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    #[storage(read)]
    fn pool(asset_pair: (AssetId, AssetId)) -> Option<ContractId>;
}

abi Exchange {
    /// Mint liquidity pool asset at current ratio and transfer to the sender.
    ///
    /// # Additional Information
    ///
    /// When liquidity is added for the first time, all deposited amounts are used to determine the ratio.
    /// When adding further liquidity, extra amounts of deposits are refunded.
    ///
    /// # Arguments
    ///
    /// * `desired_liquidity`: [u64] - The minimum amount of liquidity to add.
    /// * `deadline`: [u64] - The limit on block height for operation.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of liquidity added.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the current block height is not less than `deadline`.
    /// * When the `msg_amount` with function call is not 0.
    /// * When the `desired_liquidity` is less than `MINIMUM_LIQUIDITY`.
    /// * When asset A or B deposits are 0.
    /// * When calculated liquidity to add is less than `desired liquidity`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `5`
    /// * Writes: `6`
    #[storage(read, write)]
    fn add_liquidity(desired_liquidity: u64, deadline: u64) -> u64;

    /// Initialize contract by specifying the asset pair that makes up the pool.
    ///
    /// # Arguments
    ///
    /// * `asset_a`: [AssetId] - The unique identifier of one asset.
    /// * `asset_b`: [AssetId] - The unique identifier of the other asset.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the passed pair describes identical assets.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    #[storage(read, write)]
    fn constructor(asset_a: AssetId, asset_b: AssetId);

    /// Deposit asset to later add to the liquidity pool or withdraw.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `4`
    /// * Writes: `1`
    #[payable, storage(read, write)]
    fn deposit();

    /// Burn liquidity pool asset at current ratio and transfer asset A and asset B to the sender.
    ///
    /// # Arguments
    ///
    /// * `min_asset_a`: [u64] - The minimum amount of asset A to receive after burn.
    /// * `min_asset_b`: [u64] - minimum amount of asset B to receive after burn.
    /// * `deadline`: [u64] - The limit on block height for operation.
    ///
    /// # Returns
    ///
    /// * [RemoveLiquidityInfo] - The amount of removed amounts and burned liquidity.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When there are no liquidity pool assets to burn.
    /// * When the `msg_asset_id` does not identify the liquidity pool asset.
    /// * When `min_asset_a` or `min_asset_b` is 0.
    /// * When the current block height is not less than `deadline`.
    /// * When the `msg_amount` with function call is 0.
    /// * When the minimum amounts for asset A and asset B to receive after burn cannot be satisfied.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `3`
    /// * Writes: `2`
    #[payable, storage(read, write)]
    fn remove_liquidity(min_asset_a: u64, min_asset_b: u64, deadline: u64) -> RemoveLiquidityInfo;

    /// Swap forwarded amount of forwarded asset for other asset and transfer to sender.
    ///
    /// # Arguments
    ///
    /// * `min_output`: [Option<u64>] - The minimum output required (to protect against excessive slippage).
    /// * `deadline`: [u64] - The limit on block height for operation.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of other asset bought.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    /// * When the current block height is not less than `deadline`.
    /// * When the `msg_amount` with function call is 0.
    /// * When `min_output` is provided and is lower than the output amount.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    #[payable, storage(read, write)]
    fn swap_exact_input(min_output: Option<u64>, deadline: u64) -> u64;

    /// Swap forwarded asset for `exact_output_amount` of other asset and transfer to sender.
    ///
    /// # Arguments
    ///
    /// * `output`: [u64] - The exact output amount to receive.
    /// * `deadline`: [u64] - The limit on block height for operation.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of input asset sold.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    /// * When `output` is 0.
    /// * When the current block height is not less than ` deadline `.
    /// * When the `msg_amount` with function call is 0.
    /// * When the `msg_amount` is insufficient for swap.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    #[payable, storage(read, write)]
    fn swap_exact_output(output: u64, deadline: u64) -> u64;

    ///  Withdraw coins that have not been added to a liquidity pool yet.
    ///
    /// # Arguments
    ///
    /// * `asset`: [Asset] - The id and amount of asset to withdraw.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    /// * When the deposited amount by the sender stored in the contract is insufficient.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `4`
    /// * Writes: `1`
    #[storage(read, write)]
    fn withdraw(asset: Asset);

    /// Get current balance of the sender for a given asset on the contract.
    ///
    /// # Arguments
    ///
    /// * `asset_id`: [AssetId] - The id of the asset to get balance of.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of asset the sender has on the contract.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `4`
    #[storage(read)]
    fn balance(asset_id: AssetId) -> u64;

    /// Get the pool info of the exchange contract.
    ///
    /// # Additional Information
    ///
    /// The pool info consists of:
    /// - Identifier of asset A,
    /// - Identifier of asset B,
    /// - Asset A amount in reserves,
    /// - Asset B amount in reserves,
    /// - Liquidity pool asset supply amount.
    ///
    ///
    /// # Returns
    ///
    /// * [PoolInfo] - The pool info of the exchange contract.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `3`
    #[storage(read)]
    fn pool_info() -> PoolInfo;

    ///  Get the preview info of adding liquidity.
    ///
    /// # Additional Information
    ///
    /// The preview info consists of:
    /// - Other asset amount to input for desired liquidity,
    /// - Liquidity pool asset amount to be received.
    ///
    /// # Arguments
    ///
    /// * `asset`: [Asset] - The id and amount of asset to add.
    ///
    /// # Returns
    ///
    /// * [PreviewAddLiquidityInfo] - The preview info of adding liquidity.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `5`
    #[storage(read)]
    fn preview_add_liquidity(asset: Asset) -> PreviewAddLiquidityInfo;

    ///  Get information about the output asset for a `swap_exact_input` without doing the swap operation.
    ///
    /// # Additional Information
    ///
    /// The preview info while swapping `exact_input` of input asset consists of:
    /// - The minimum amount of output asset to receive,
    /// - Whether the output asset reserves are sufficient for the swap or not.
    ///
    /// # Arguments
    ///
    /// * `exact_input_asset`: [Asset] - The asset to sell.
    ///
    /// # Returns
    ///
    /// * [PreviewSwapInfo] - The preview info of swapping `exact_input` of input asset.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    #[storage(read)]
    fn preview_swap_exact_input(exact_input_asset: Asset) -> PreviewSwapInfo;

    ///  Get information about the input asset for a `swap_exact_output` without doing the swap operation.
    ///
    /// # Additional Information
    ///
    /// The preview info while swapping to get `exact_output` amount of output asset consists of:
    /// - The maximum amount of input asset to forward,
    /// - Whether the input asset reserves are sufficient for the swap or not.
    ///
    /// # Arguments
    ///
    /// * `exact_output_asset`: `Asset` - The asset to buy.
    ///
    /// # Returns
    ///
    /// * [PreviewSwapInfo] - The preview info of swapping to get `exact_output` amount of output asset.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    /// * When the `exact_output` is less than the reserve amount of the output asset.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    #[storage(read)]
    fn preview_swap_exact_output(exact_output_asset: Asset) -> PreviewSwapInfo;
}
