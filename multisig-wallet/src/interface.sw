library interface;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};

use data_structures::{SignatureData, User};

abi MultiSignatureWallet {
    /// The constructor initializes the necessary values and unlocks further functionlity
    ///
    /// # Panics
    ///
    /// - When the constructor is called more than once
    /// - When the user address is the 0th address (0x00000...)
    /// - When the threshold is set to 0
    /// - When an owner has an approval weight of 0
    #[storage(read, write)]
    fn constructor(users: Vec<User>, threshold: u64);

    /// Executes a Tx formed from the `to, `value` and `data` parameters if the signatures meet the
    /// threshold requirement
    ///
    /// # Panics
    ///
    /// - When the constructor has not been called to initialize the contract
    /// - When the public key cannot be recovered from a signature
    /// - When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...)
    /// - When the total approval count is less than the required threshold for execution
    #[storage(read, write)]
    fn execute_transaction(to: Identity, value: u64, data: b256, signatures_data: Vec<SignatureData>);

    /// Transfers assets to outputs & contracts if the signatures meet the threshold requirement
    ///
    /// # Panics
    ///
    /// - When the constructor has not been called to initialize the contract
    /// - When the amount of the asset being sent is greater than the balance in the contract
    /// - When the public key cannot be recovered from a signature
    /// - When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...)
    /// - When the total approval count is less than the required threshold for execution
    #[storage(read, write)]
    fn transfer(to: Identity, asset_id: ContractId, value: u64, data: b256, signatures_data: Vec<SignatureData>);

    /// Returns the current nonce in the contract
    /// Used to check the nonce and create a Tx via transaction_hash()
    #[storage(read)]
    fn nonce() -> u64;

    /// Returns the balance of the specified asset_id for this contract
    fn balance(asset_id: ContractId) -> u64;

    /// Takes in transaction data and hashes it into a unique tx hash
    /// Used for verification of message
    fn transaction_hash(to: Identity, value: u64, data: b256, nonce: u64) -> b256;
}
