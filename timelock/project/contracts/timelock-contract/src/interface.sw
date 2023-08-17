library;

use ::data_structures::{Asset, ExecutionRange};
use std::bytes::Bytes;

abi Timelock {
    /// Deletes a transaction from storage preventing execution.
    ///
    /// # Arguments
    ///
    /// * `id`: [b256] - The hash of a transaction.
    ///
    /// # Reverts
    ///
    /// * When a non-admin calls the function.
    /// * When the transaction `id` does not exist.
    #[storage(read, write)]
    fn cancel(id: b256);

    /// Makes a call to execute a transaction.
    ///
    /// # Arguments
    ///
    /// * `recipient`: [Identity] - The target of the call.
    /// * `asset`: [Option<Asset>] - Asset that is transferred.
    /// * `data`: [Option<Bytes>] - Payload associated with the transaction.
    /// * `timestamp`: [u64] - Time after which the transaction may be executed.
    ///
    /// # Reverts
    ///
    /// * When a non-admin calls the function.
    /// * When the transaction id does not exist.
    /// * When the execution occurs outside of the available time range.
    /// * When the contract does not have enough of the asset to transfer.
    #[storage(read, write)]
    fn execute(recipient: Identity, asset: Option<Asset>, data: Option<Bytes>, timestamp: u64);

    /// Stores a transaction for future execution.
    ///
    /// # Arguments
    ///
    /// * `recipient`: [Identity] - The target of the call.
    /// * `asset`: [Option<Asset>] - Native asset that will be transferred.
    /// * `data`: [Option<Bytes>] - Payload associated with the transaction.
    /// * `timestamp`: [u64] - Time after which the transaction may be executed.
    ///
    /// # Reverts
    ///
    /// * When a non-admin calls the function.
    /// * When the transaction id already exists.
    /// * When the timestamp is outside of the valid MINIMUM_DELAY / MAXIMUM_DELAY range.
    #[storage(read, write)]
    fn queue(recipient: Identity, asset: Option<Asset>, data: Option<Bytes>, timestamp: u64);
}

abi Info {
    /// Returns the amount of `asset_id` in the contract.
    ///
    /// # Arguments
    ///
    /// * `asset_id`: [ContractId] - The identifier of an asset.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of `asset_id` in the contract.
    fn balance(asset_id: ContractId) -> u64;

    /// Returns the (MINIMUM_DELAY, MAXIMUM_DELAY) values.
    fn delays() -> (u64, u64);

    /// Returns an optional time range for which a transaction may be executed.
    ///
    /// # Arguments
    ///
    /// * `id`: [Option<ExecutionRange>] The hash of a transaction.
    ///
    /// # Returns
    ///
    /// * [Option<ExecutionRange>] - The time range for which a transaction may be executed.
    #[storage(read)]
    fn queued(id: b256) -> Option<ExecutionRange>;

    /// Hashes the transaction arguments and returns the transaction id.
    ///
    /// # Arguments
    ///
    /// - `recipient`: [Identity] - The target of the call.
    /// - `asset`: [Option<Asset>] - Native asset that will be transferred.
    /// - `data`: [Option<Bytes>] - Payload associated with transaction.
    /// - `timestamp`: [u64] - Time after which the transaction may be executed.
    ///
    /// # Returns
    ///
    /// * [b256] - The hash of the transaction arguments.
    fn transaction_hash(recipient: Identity, asset: Option<Asset>, data: Option<Bytes>, timestamp: u64) -> b256;
}
