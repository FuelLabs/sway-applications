library contract_abi;

dep data_structures;

use std::{address::Address, b512::B512, contract_id::ContractId, identity::Identity};

use data_structures::User;

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

    #[storage(read, write)]
    fn execute_transaction(to: Identity, value: u64, data: b256, signatures: [B512; 25]);

    #[storage(read, write)]
    fn transfer(to: Identity, asset_id: ContractId, value: u64, data: b256, signatures: [B512; 25]);

    #[storage(read)]
    fn is_owner(owner: Address) -> bool;

    #[storage(read)]
    fn nonce() -> u64;

    fn balance(asset_id: ContractId) -> u64;
    fn transaction_hash(to: Identity, value: u64, data: b256, nonce: u64) -> b256;
}
