contract;

use std::{
    address::Address,
    assert::assert,
    b512::B512,
    chain::log_b256,
    context::call_frames::contract_id,
    contract_id::ContractId,
    ecr::{EcRecoverError, ec_recover_address},
    hash::{HashMethod, hash_pair, hash_u64, hash_value},
    result::*,
    revert::revert,
    storage::{get, store}
};

abi MultiSignatureWallet {
    fn constructor(owner1: Address, owner2: Address) -> bool;
    fn executeTransaction(tx_hash: b256, signatures: [B512;
    2]) -> bool;
    fn is_owner(owner: Address) -> bool;
    fn get_transaction_hash(to: ContractId, value: u64, data: b256) -> b256;
}

storage {
    /// Value used to keep track of the number of Tx and prevent Tx duplication when hashing
    nonce: u64,
}

impl MultiSignatureWallet for Contract {
    /// Initializes the contract by setting the owners and the nonce
    ///
    /// # Example
    ///
    /// ```
    /// // Init contract here ...
    ///
    /// let owner1 = ~Address::from(/* some b256 here */);
    /// let owner2 = ~Address::from(/* some b256 here */);
    /// contract.constructor(owner1, owner2).call().await.unwrap();
    /// ```
    fn constructor(owner1: Address, owner2: Address) -> bool {
        assert(storage.nonce == 0);

        // TODO: when vectors are implemented change owners to be a Vec<Address>
        store(owner1.value, true);
        store(owner2.value, true);

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
    /// let tx_hash = contract.get_transaction_hash(destination_contract, value, data).call.await.unwrap();
    ///
    /// let signatures = [~B512::new(), ~B512::new()];
    /// contract.executeTransaction(tx_hash, signatures).call().await.unwrap();
    /// ```
    fn executeTransaction(tx_hash: b256, signatures: [B512;
    2]) -> bool {
        _assert_is_initialized(storage.nonce);
        _assert_not_executed(get::<bool>(tx_hash));

        // The signers must have increasing values in order to check for duplicates or a zero-value
        let mut previous_signer: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

        // TODO: change to Vec<B512> once implemented and then iterate instead of hardcoding length
        let mut index = 0;
        while index < 2 {
            let signer: Result<Address, EcRecoverError> = ec_recover_address(signatures[index], tx_hash);
            if let Result::Err = signer {
                revert(0);
            };

            let signer = signer.unwrap();
            _assert_is_owner(get(signer.value));
            // assert(previous_signer < signer.value);  // TODO: "lt" is not implemented for b256 atm
            previous_signer = signer.value;
        }

        // Prevent multiple executions of the same Tx
        store(tx_hash, true);

        // Increment nonce so that get_transaction_hash() returns a different hash if the same values are used
        storage.nonce = storage.nonce + 1;
        log_b256(tx_hash);

        // TODO: Execute (https://github.com/FuelLabs/sway-applications/issues/6 and/or https://github.com/FuelLabs/sway-applications/issues/22)

        true
    }

    /// Takes in transaction data and hashes it into a unique tx hash
    ///
    /// After the hash is generated each signer can sign the hash in order to create
    /// signatures that can be passed into executeTransaction()
    ///
    /// # Example
    ///
    /// ```
    /// // Init contract here ...
    ///
    /// let destination_contract = ~Contract::from(/* some b256 here */);
    /// let value = 100;
    /// let data = /* some b256 here? */;
    /// let tx_hash = contract.get_transaction_hash(destination_contract, value, data).call.await.unwrap();
    /// ```
    fn get_transaction_hash(to: ContractId, value: u64, data: b256) -> b256 {
        _assert_is_initialized(storage.nonce);
        // TODO: data > b256?
        // TODO: this is probably not following the EIP-191 signing standard. What do?
        let to_hash = hash_value(to.value, HashMethod::Keccak256);
        let data_hash = hash_value(data, HashMethod::Keccak256);
        let value_hash = hash_u64(value, HashMethod::Keccak256);
        let nonce_hash = hash_u64(storage.nonce, HashMethod::Keccak256);

        let id = contract_id();
        hash_pair(id.value, hash_pair(to_hash, hash_pair(value_hash, hash_pair(data_hash, nonce_hash, HashMethod::Keccak256), HashMethod::Keccak256), HashMethod::Keccak256), HashMethod::Keccak256)
    }

    /// Returns a boolean value indicating if the given address is an owner in the contract
    fn is_owner(address: Address) -> bool {
        _assert_is_initialized(storage.nonce);
        get(address.value)
    }
}

/// Assertion used to ensure that the value stored for the owner is true i.e. they are an owner in the contract
fn _assert_is_owner(state: bool) {
    assert(state);
}

/// Assertion used to ensure that the contract has called the constructor to initialize the values
fn _assert_is_initialized(nonce: u64) {
    assert(nonce != 0);
}

/// Assertion used to ensure that a Tx has not been submitted and executed in executeTransaction()
/// This prevents multiple uses of the same Tx
fn _assert_not_executed(executed: bool) {
    assert(!executed);
}
