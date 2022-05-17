contract;

use std::{
    address::Address,
    assert::require,
    b512::B512,
    chain::log_b256,
    constants::ZERO,
    context::call_frames::contract_id,
    contract_id::ContractId,
    ecr::{EcRecoverError, ec_recover_address},
    hash::sha256,
    result::*,
    revert::revert,
    storage::{get, store}
};

abi MultiSignatureWallet {
    fn constructor(owner1: Address, owner2: Address) -> bool;
    fn executeTransaction(to: ContractId, value: u64, data: b256, signatures: [B512;
    2]) -> bool;
    fn is_owner(owner: Address) -> bool;
    fn get_transaction_hash(to: ContractId, value: u64, data: b256, nonce: u64) -> b256;
}

enum Error {
    CannotReinitialize: (),
    IncorrectSignerOrdering: (),
    NotAnOwner: (),
    NotInitialized: (),
}

struct Tx {
    destination: b256,
    value: u64,
    data: b256,
    nonce: u64,
    contact_identifier: b256,
}

storage {
    /// Used to add entropy into hashing of Tx to decrease the probability of collisions
    nonce: u64,

    /// The number of approvals (signatures * weight) required in order to execture a Tx
    threshold: u64,
}

impl MultiSignatureWallet for Contract {
    /// Initializes the contract by setting the owners, owner weightings and initializes the nonce
    ///
    /// # Example
    ///
    /// ```
    /// // Init contract here ...
    ///
    /// let owner1 = ~Address::from(/* some b256 here */);
    /// let owner2 = ~Address::from(/* some b256 here */);
    ///
    /// contract.constructor(owner1, owner2).call().await.unwrap();
    /// ```
    fn constructor(owner1: Address, owner2: Address) -> bool {
        require(storage.nonce == 0, Error::CannotReinitialize);

        // TODO: when vectors are implemented change owners to be a Vec<Address>
        store(owner1.value, true);
        store(owner2.value, true);

        // TODO: Weighted signers https://github.com/FuelLabs/sway-applications/issues/4

        storage.nonce = 1;
        true
    }

    /// Executes a Tx if the required signatures meet the restrictions on ownership and threshold approval
    ///
    /// # Example
    ///
    /// ```
    /// // Init contract here ...
    ///
    /// let destination_contract = ~Contract::from(/* some b256 here */);
    /// let value = 100;
    /// let data = /* some b256 here? */;
    ///
    /// // Dummy signatures here, use real ones for it to work
    /// let signatures = [~B512::new(), ~B512::new()];
    ///
    /// contract.executeTransaction(destination_contract, value, data, signatures).call().await.unwrap();
    /// ```
    fn executeTransaction(to: ContractId, value: u64, data: b256, signatures: [B512;
    2]) -> bool {
        require(storage.nonce != 0, Error::NotInitialized);

        let tx_hash = _get_transaction_hash(to, value, data, storage.nonce, contract_id());

        // The signers must have increasing values in order to check for duplicates or a zero-value
        let mut previous_signer: b256 = ZERO;

        // TODO: change to Vec<B512> once implemented and then iterate instead of hardcoding length
        let mut index = 0;
        while index < 2 {
            let signer: Result<Address, EcRecoverError> = ec_recover_address(signatures[index], tx_hash);
            if let Result::Err = signer {
                // TODO: it would be nice to use a "require()" with the enum log from EcRecoverError::UnrecoverablePublicKey
                revert(0);
            };

            let signer = signer.unwrap();
            require(get::<bool>(signer.value), Error::NotAnOwner);
            require(previous_signer < signer.value, Error::IncorrectSignerOrdering);
            previous_signer = signer.value;
        }

        // TODO: Approval threshold https://github.com/FuelLabs/sway-applications/issues/3

        storage.nonce = storage.nonce + 1;
        log_b256(tx_hash);

        // TODO: Execute (https://github.com/FuelLabs/sway-applications/issues/6 and/or https://github.com/FuelLabs/sway-applications/issues/22)

        true
    }

    /// Takes in transaction data and hashes it into a unique tx hash
    ///
    /// Used for verification of message
    ///
    /// # Example
    ///
    /// ```
    /// // Init contract here ...
    ///
    /// let destination_contract = ~Contract::from(/* some b256 here */);
    /// let value = 100;
    /// let data = /* some b256 here? */;
    /// let nonce = 42;
    ///
    /// let tx_hash = contract.get_transaction_hash(destination_contract, value, data, nonce).call().await.unwrap();
    /// ```
    fn get_transaction_hash(to: ContractId, value: u64, data: b256, nonce: u64) -> b256 {
        // TODO: data > b256?
        _get_transaction_hash(to, value, data, nonce, contract_id())
    }

    /// Returns a boolean value indicating if the given address is an owner in the contract
    fn is_owner(address: Address) -> bool {
        require(storage.nonce != 0, Error::NotInitialized);
        get(address.value)
    }
}

fn _get_transaction_hash(to: ContractId, value: u64, data: b256, nonce: u64, self_id: ContractId) -> b256 {
    sha256(Tx {
        contact_identifier: self_id.value, destination: to.value, value, data, nonce
    })
}
