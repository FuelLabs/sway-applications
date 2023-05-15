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
    /// Mint liquidity pool asset at current ratio and transfer to the sender.
    ///
    /// When liquidity is added for the first time, all deposited amounts are used to determine the ratio.
    /// When adding further liquidity, extra amounts of deposits are refunded.
    ///
    /// # Arguments
    ///
    /// - `desired_liquidity` - minimum amount of liquidity to add
    /// - `deadline` - limit on block height for operation
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the current block height is not less than `deadline`
    /// * When the `msg_amount` with function call is not 0
    /// * When the `desired_liquidity` is less than `MINIMUM_LIQUIDITY`
    /// * When asset A or B deposits are 0
    /// * When calculated liquidity to add is less than `desired liquidity`
    #[storage(read, write)]
    fn add_liquidity(desired_liquidity: u64, deadline: u64) -> u64;

    /// Initialize contract by specifying the asset pair that makes up the pool.
    ///
    /// # Arguments
    ///
    /// - `asset_a` - unique identifier of one asset
    /// - `asset_b` - unique identifier of the other asset
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the passed pair describes identical assets
    #[storage(read, write)]
    fn constructor(asset_a: ContractId, asset_b: ContractId);

    /// Deposit asset to later add to the liquidity pool or withdraw.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the `msg_asset_id` does not identify asset A or asset B
    #[payable, storage(read, write)]
    fn deposit();

    /// Burn liquidity pool asset at current ratio and transfer asset A and asset B to the sender.
    ///
    /// # Arguments
    ///
    /// - `deadline` - limit on block height for operation
    /// - `min_asset_a` - minimum amount of asset A to receive after burn
    /// - `min_asset_b` - minimum amount of asset B to receive after burn
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When there are no liquidity pool assets to burn
    /// * When the `msg_asset_id` does not identify the liquidity pool asset
    /// * When `min_asset_a` or `min_asset_b` is 0
    /// * When the current block height is not less than `deadline`
    /// * When the `msg_amount` with function call is 0
    /// * When the minimum amounts for asset A and asset B to receive after burn cannot be satisfied
    #[payable, storage(read, write)]
    fn remove_liquidity(min_asset_a: u64, min_asset_b: u64, deadline: u64) -> RemoveLiquidityInfo;

    /// Swap forwarded amount of forwarded asset for other asset and transfer to sender.
    ///
    /// # Arguments
    ///
    /// - `min_output` - minimum output required (to protect against excessive slippage)
    /// - `deadline` - limit on block height for operation
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the `msg_asset_id` does not identify asset A or asset B
    /// * When the current block height is not less than `deadline`
    /// * When the `msg_amount` with function call is 0
    /// * When `min_output` is provided and is lower than the output amount
    #[payable, storage(read, write)]
    fn swap_exact_input(min_output: Option<u64>, deadline: u64) -> u64;

    /// Swap forwarded asset for `exact_output_amount` of other asset and transfer to sender.
    ///
    /// Refund any extra input amount.
    ///
    /// # Arguments
    ///
    /// - `output` - the exact output amount to receive
    /// - `deadline` - limit on block height for operation
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the `msg_asset_id` does not identify asset A or asset B
    /// * When `output` is 0
    /// * When the current block height is not less than ` deadline `
    /// * When the `msg_amount` with function call is 0
    /// * When the `msg_amount` is insufficient for swap
    #[payable, storage(read, write)]
    fn swap_exact_output(output: u64, deadline: u64) -> u64;

    /// Withdraw coins that have not been added to a liquidity pool yet.
    ///
    /// # Arguments
    ///
    /// - `asset` - id and amount of asset to withdraw
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the `msg_asset_id` does not identify asset A or asset B
    /// * When the deposited amount by the sender stored in the contract is insufficient
    #[storage(read, write)]
    fn withdraw(asset: Asset);

    /// Get current balance of the sender for a given asset on the contract.
    ///
    /// # Arguments
    ///
    /// - `asset_id` - asset to get balance of
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    #[storage(read)]
    fn balance(asset_id: ContractId) -> u64;

    /// Get the pool info of the exchange contract.
    ///
    /// The pool info consists of:
    /// - Identifier of asset A,
    /// - Identifier of asset B,
    /// - Asset A amount in reserves,
    /// - Asset B amount in reserves,
    /// - Liquidity pool asset supply amount.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    #[storage(read)]
    fn pool_info() -> PoolInfo;

    /// Get the preview info of adding liquidity.
    ///
    /// The preview info consists of:
    /// - Other asset amount to input for desired liquidity,
    /// - Liquidity pool asset amount to be received.
    ///
    /// # Arguments
    ///
    /// - `asset` - id and amount of asset to add
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    #[storage(read)]
    fn preview_add_liquidity(asset: Asset) -> PreviewAddLiquidityInfo;

    /// Get information about the output asset for a `swap_exact_input` without doing the swap operation.
    ///
    /// The preview info while swapping `exact_input` of input asset consists of:
    /// - The minimum amount of output asset to receive,
    /// - Whether the output asset reserves are sufficient for the swap or not.
    ///
    /// # Arguments
    ///
    /// - `exact_input_asset` - the asset to sell
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the `msg_asset_id` does not identify asset A or asset B
    #[storage(read)]
    fn preview_swap_exact_input(exact_input_asset: Asset) -> PreviewSwapInfo;

    /// Get information about the input asset for a `swap_exact_output` without doing the swap operation.
    ///
    /// The preview info while swapping to get `exact_output` amount of output asset consists of:
    /// - The maximum amount of input asset to forward,
    /// - Whether the input asset reserves are sufficient for the swap or not.
    ///
    /// # Arguments
    ///
    /// - `exact_output_asset` - the asset to buy
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the `msg_asset_id` does not identify asset A or asset B
    /// * When the `exact_output` is less than the reserve amount of the output asset
    #[storage(read)]
    fn preview_swap_exact_output(exact_output_asset: Asset) -> PreviewSwapInfo;
}
