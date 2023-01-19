library interface;

dep data_structures;

use data_structures::{PoolInfo, PreviewAddLiquidityInfo, PreviewSwapInfo, RemoveLiquidityInfo};

abi AMM {
    /// Initialize the AMM by specifying the exchange contract bytecode root, for security.
    ///
    /// # Arguments
    ///
    /// - `exchange_bytecode_root` - bytecode root of the intended implementation of the exchange ABI
    ///
    /// # Reverts
    ///
    /// * When the AMM has already been initialized
    #[storage(read, write)]
    fn initialize(exchange_bytecode_root: ContractId);

    /// Add an (asset pair, exchange contract ID) mapping to the storage.
    ///
    /// # Arguments
    ///
    /// - `asset_pair` - pair of assets that make up the pool
    /// - `pool` - exchange contract that defines the pool for a given `asset_pair`
    ///
    /// # Reverts
    ///
    /// * When the AMM contract has not been initialized
    /// * When the bytecode root of `pool` does not match the bytecode root of the intended exchange contract
    /// * When the pool info of the exchange contract with the given address does not consist of the given asset pair
    #[storage(read, write)]
    fn add_pool(asset_pair: (ContractId, ContractId), pool: ContractId);

    /// For the given asset pair, get the exchange contract, i.e., the pool that consists of the asset pair.
    ///
    /// # Arguments
    ///
    /// - `asset_pair` - pair of assets that make up the pool
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
    /// - `pair` - unique identifiers of the asset pair, i.e., asset A and asset B
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the passed pair describes identical assets
    #[storage(read, write)]
    fn constructor(pair: (ContractId, ContractId));

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
    /// - `amount` - the amount of coins to withdraw
    /// - `asset` - asset to withdraw
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the `msg_asset_id` does not identify asset A or asset B
    /// * When the deposited amount by the sender stored in the contract is insufficient
    #[storage(read, write)]
    fn withdraw(amount: u64, asset: ContractId);

    /// Get current balance of the sender for a given asset on the contract.
    ///
    /// # Arguments
    ///
    /// - `asset` - asset to get balance of
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    #[storage(read)]
    fn balance(asset: ContractId) -> u64;

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
    /// - `amount` - amount of an asset to add
    /// - `asset` - asset to add
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    #[storage(read)]
    fn preview_add_liquidity(amount: u64, asset: ContractId) -> PreviewAddLiquidityInfo;

    /// Get information about the output asset for a `swap_exact_input` without doing the swap operation.
    ///
    /// The preview info while swapping `exact_input` of input asset consists of:
    /// - The minimum amount of output asset to receive,
    /// - Whether the output asset reserves are sufficient for the swap or not.
    ///
    /// # Arguments
    ///
    /// - `exact_input` - the amount to input
    /// - `input_asset` - asset to input
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the `msg_asset_id` does not identify asset A or asset B
    #[storage(read)]
    fn preview_swap_exact_input(exact_input: u64, input_asset: ContractId) -> PreviewSwapInfo;

    /// Get information about the input asset for a `swap_exact_output` without doing the swap operation.
    ///
    /// The preview info while swapping to get `exact_output` amount of output asset consists of:
    /// - The maximum amount of input asset to forward,
    /// - Whether the input asset reserves are sufficient for the swap or not.
    ///
    /// # Arguments
    ///
    /// - `exact_output` - the desired amount of other asset to receive after swap
    /// - `output_asset` - asset to output
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the `msg_asset_id` does not identify asset A or asset B
    /// * When the `exact_output` is less than the reserve amount of the output asset
    #[storage(read)]
    fn preview_swap_exact_output(exact_output: u64, output_asset: ContractId) -> PreviewSwapInfo;
}
