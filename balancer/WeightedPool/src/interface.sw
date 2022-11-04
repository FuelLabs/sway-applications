library interface;

dep data_structures;
use data_structures::UserData;

use std::{address::Address, contract_id::ContractId, vec::Vec};

abi WeightedPool {
    // Called by the Vault when a user calls `joinPool` to add liquidity to this Pool. Returns how many of
    // each registered token the user should provide, as well as the amount of protocol fees the Pool owes to the Vault.
    // The Vault will then take tokens from `sender` and add them to the Pool's balances, as well as collect
    // the reported amount in protocol fees, which the pool should calculate based on `protocolSwapFeePercentage`.
    //
    // Protocol fees are reported and charged on join events so that the Pool is free of debt whenever new users join.
    //
    // `userData` contains any pool-specific instructions needed to perform the calculations, such as the type of
    // join (e.g., proportional given an amount of pool shares, single-asset, multi-asset, etc.)
    //
    // Contracts implementing this function should check that the caller is indeed the Vault before performing any
    // state-changing operations, such as minting pool shares.
    //
    // # Arguments
    //
    // * `poolId` - The id of the pool
    // * `sender` - the account performing the join (from which tokens will be withdrawn)
    // * `recipient` - The account performing the join (from which tokens will be withdrawn)
    // * `balances` - The account designated to receive any benefits (typically pool shares)
    // * `lastChangeBlock` - The last block in which *any* of the Pool's registered tokens last changed its total balance.
    // * `protocolSwapFeePercentage` - The percentage of the total balance that
    // * `userData` - contains any pool-specific instructions needed to perform the calculations, such as the type of join (e.g., proportional given an amount of pool shares, single-asset, multi-asset, etc.)
    //
    // # Returns
    //
    // * The array of deposit amount
    // * The array of fee of the due protocol fee amounts
    #[storage(read, write)]
    fn on_join_pool(
        poolId: b256,
        // sender: ContractId,
        // recipient: ContractId,
        // balances: Vec<u64>,
        sender: b256,
        recipient: b256,
        balances_array: [u64; 2],
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: UserData,
    );

    // -> (Vec<u64>, Vec<u64>);
    // Called by the Vault when a user calls `IVault.exitPool` to remove liquidity from this Pool. Returns how many
    // tokens the Vault should deduct from the Pool's balances, as well as the amount of protocol fees the Pool owes
    // to the Vault. The Vault will then take tokens from the Pool's balances and send them to `recipient`,
    // as well as collect the reported amount in protocol fees, which the Pool should calculate based on
    // `protocolSwapFeePercentage`.
    //
    // Protocol fees are charged on exit events to guarantee that users exiting the Pool have paid their share.
    //
    // Contracts implementing this function should check that the caller is indeed the Vault before performing any
    // state-changing operations, such as burning pool shares.
    //
    // # Arguments
    //
    // * `poolId` - The id of the pool
    // * `sender` - the account performing the join (from which tokens will be withdrawn)
    // * `recipient` - The account performing the join (from which tokens will be withdrawn)
    // * `balances` - The account designated to receive any benefits (typically pool shares)
    // * `lastChangeBlock` - The last block in which *any* of the Pool's registered tokens last changed its total balance.
    // * `protocolSwapFeePercentage` - The percentage of the total balance that
    // * `userData` - contains any pool-specific instructions needed to perform the calculations, such as the type of join (e.g., proportional given an amount of pool shares, single-asset, multi-asset, etc.)
    //
    // # Returns
    //
    // * The array of deposit amount
    // * The array of fee of the due protocol fee amounts
    #[storage(read, write)]
    fn on_exit_pool(
        poolId: b256,
        sender: ContractId,
        recipient: ContractId,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: UserData,
    ) -> (Vec<u64>, Vec<u64>);

    // Set the swap fee percentage.
    // This is a permissioned function, and disabled if the pool is paused. The swap fee must be within the
    // bounds set by MIN_SWAP_FEE_PERCENTAGE/MAX_SWAP_FEE_PERCENTAGE. Emits the SwapFeePercentageChanged event.
    //
    // # Arguments
    //
    // * `swapFeePercentage` - The new amount need to be set
    #[storage(read, write)]
    fn set_swap_fee_percentage(swapFeePercentage: u64);

    // "Dry run" `onJoinPool`.
    // Returns the amount of BPT that would be granted to `recipient` if the `onJoinPool` hook were called by the
    // Vault with the same arguments, along with the number of tokens `sender` would have to supply.
    //
    // Like `IVault.queryBatchSwap`, this function is not view due to internal implementation details: the caller must
    // explicitly use eth_call instead of eth_sendTransaction.
    #[storage(read, write)]
    fn query_join(
        poolId: b256,
        sender: ContractId,
        arecipient: ContractId,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: UserData,
    ) -> (u64, Vec<u64>);
    #[storage(read, write)]
    fn query_exit(
        poolId: b256,
        sender: ContractId,
        arecipient: ContractId,
        balances: Vec<u64>,
        lastChangeBlock: u64,
        protocolSwapFeePercentage: u64,
        userData: UserData,
    ) -> (u64, Vec<u64>);

    // # Arguments
    //
    // * `coins` - The amount of coins to send to the contract
    // * `asset_id` - The contract id of the asset
    // * `target` - The contract id of the target
    fn force_transfer_coins(coins: u64, asset_id: ContractId, target: ContractId);

    // # Arguments
    //
    // * `coins` - The amount of coins to send to the contract
    // * `asset_id` - The contract id of the asset
    // * `target` - The address of the target
    fn transfer_coins_to_output(coins: u64, asset_id: ContractId, recipient: Address);

    // # Returns
    //
    // * The array of normilized weights
    fn get_normalized_weights() -> Vec<u64>;

    // # Returns
    //
    // * The address of the vault
    #[storage(read)]
    fn get_vault() -> ContractId;

    // // # Returns
    // //
    // // * The swap fee percentage
    // #[storage(read)]fn get_swap_fee_percentage() -> u64;
}
