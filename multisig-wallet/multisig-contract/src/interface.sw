library;

use ::data_structures::{
    hashing::{
        TransactionParameters,
        TypeToHash,
    },
    signatures::SignatureInfo,
    user::User,
};

abi MultiSignatureWallet {
    /// The constructor initializes the necessary values and unlocks further functionality.
    ///
    /// # Arguments
    ///
    /// * `users`: [Vec<User>] - The users of the multisig, who can sign transactions to add their approval.
    ///
    /// # Reverts
    ///
    /// * When the constructor has already been called.
    /// * When `THRESHOLD` is zero [u64].
    /// * When `THRESHOLD` is greater the sum of the weights from users in `users` [Vec<User>].
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `4`
    #[storage(read, write)]
    fn constructor(users: Vec<User>);

    /// This function executes either a transfer or a contract call depending on the presence of `contract_call_params`.
    ///
    /// # Arguments
    ///
    /// * `signatures`: [Vec<SignatureInfo>] - The information for each user's signature for a specific transaction.
    /// * `target`: [Identity] - The target of the transaction.
    /// * `transaction_parameters`: [TransactionParameters] - The parameters of the transaction.
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize the contract.
    /// * When attempting to transfer with `transfer_params.value` as [Option::None].
    /// * When the amount of the asset being sent is greater than the balance in the contract.
    /// * When the public key cannot be recovered from a signature.
    /// * When the recovered addresses in `count_approvals `are not in ascending order (0x1 < 0x2 < 0x3...) [b256].
    /// * When the total approval count is less than the required threshold for execution.
    /// * When attempting to call when `target` is not a [Identity::ContractId].
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `3`
    /// * Writes: `2`
    #[storage(read, write)]
    fn execute_transaction(
        signatures: Vec<SignatureInfo>,
        target: Identity,
        transaction_parameters: TransactionParameters,
    );

    /// Updates the threshold required for execution.
    ///
    /// # Arguments
    ///
    /// * `signatures``: [Vec<SignatureInfo>] - The information for each user's signature for a specific transaction.
    /// * `threshold`: [u64] - The number of approvals required to enable a transaction to be sent.
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize the contract.
    /// * When the threshold is zero.
    /// * When the threshold is a value greater than the sum of the weights.
    /// * When the public key cannot be recovered from a signature.
    /// * When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...) [b256].
    /// * When the total approval count is less than the required threshold for execution.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `3`
    /// * Writes: `2`
    #[storage(read, write)]
    fn set_threshold(signatures: Vec<SignatureInfo>, threshold: u64);

    /// Changes the approval weights of a user in the contract.
    ///
    /// # Arguments
    ///
    /// * `signatures``: [Vec<SignatureInfo>] - The information for each user's signature for a specific transaction.
    /// * `user` : [User] - The user of the multisig, who can sign transactions to add their approval.
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize the contract.
    /// * When the public key cannot be recovered from a signature.
    /// * When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...) [b256].
    /// * When the total approval count is less than the required threshold for execution.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `5`
    /// * Writes: `3`
    #[storage(read, write)]
    fn set_weight(signatures: Vec<SignatureInfo>, user: User);
}

abi Info {
    /// Returns the approval weight of a user.
    ///
    /// # Arguments
    ///
    /// * `user` : [b256] - User of the contract.
    ///
    /// # Returns
    ///
    /// * [u64] - The number of approvals associated with the `user`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    #[storage(read)]
    fn approval_weight(user: b256) -> u64;

    /// Returns the contract's balance of the specified asset.
    ///
    /// # Arguments
    ///
    /// * `asset_id` : [AssetId] - The AssetId of the asset to check that balance of.
    ///
    /// # Returns
    ///
    /// * [u64] - The multisig wallet's balance of `asset_id`.
    fn balance(asset_id: AssetId) -> u64;

    /// Takes a struct comprised of transaction data and hashes it.
    ///
    /// # Additional Information
    ///
    /// The struct will be a variant of [TypeToHash].
    ///
    /// # Arguments
    ///
    /// * `type_to_hash` : [TypeToHash] - The struct to hash.
    ///
    /// # Returns
    ///
    /// * [b256] - The hash.
    fn compute_hash(type_to_hash: TypeToHash) -> b256;

    /// Returns the current nonce.
    ///
    /// # Returns
    ///
    /// * [u64] - The current nonce.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    #[storage(read)]
    fn nonce() -> u64;

    /// Returns the current threshold.
    ///
    /// # Returns
    ///
    /// * [u64] - The current number of approvals required in order to execute a transaction.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    #[storage(read)]
    fn threshold() -> u64;
}
