contract;

use std::{address::Address, assert::assert, chain::{auth::{AuthError, Sender, msg_sender}, log_b256}, context::call_frames::contract_id, contract_id::ContractId, hash::{HashMethod, hash_pair, hash_u64, hash_value}, panic::panic, result::*, storage::{get, store}};

abi MultiSignatureWallet {
    fn constructor(owner1: Address, owner2: Address) -> bool;
    fn submitTransaction(tx_hash: b256) -> bool;
    fn approveTransaction(tx_hash: b256) -> bool;
    fn revokeTransaction(tx_hash: b256) -> bool;
    fn executeTransaction(tx_hash: b256) -> bool;
    fn is_owner(owner: Address) -> bool;
    fn has_approved(tx_hash: b256, owner: Address) -> bool;
    fn get_transaction_hash(to: ContractId, value: u64, data: b256, nonce: u64) -> b256;
    fn get_transaction_data(tx_hash: b256) -> (u64, bool);
}

storage {
    /// The state is used as a gatekeeper for unlocking functionality post initialization
    state: u64,
    /// The sentinel is used as a dummy value to store under a newly submitted tx hash
    /// so that we can
    ///     - use the tx in other functions
    ///     - prevent a resubmission of the same tx
    tx_sentinel: u64,
}

/// TODO: proper documentation instead of temporary filler comments

impl MultiSignatureWallet for Contract {
    /// Initializes the contract by setting the owners, state and a sentinel value for the tx
    fn constructor(owner1: Address, owner2: Address) -> bool {
        assert(storage.state == 0);

        // TODO: when vectors are implemented change owners to be a Vec<Address>
        store(owner1.value, true);
        store(owner2.value, true);

        storage.state = 1;
        storage.tx_sentinel = 1;
        true
    }

    /// Takes a transaction hash and stores in it storage
    fn submitTransaction(tx_hash: b256) -> bool {
        _is_initialized(storage.state);

        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            let status: bool = get(address.value);
            _is_owner(status);

            // When a Tx has not been submitted its default value should be 0.
            // Make sure we are not overwritting a previous Tx
            let tx_data: u64 = get(tx_hash);
            assert(tx_data == 0);

            // Mark the first submission as a non-zero value
            store(tx_hash, storage.tx_sentinel);
            log_b256(tx_hash);
        } else {
            panic(0);
        };

        true
    }

    /// Takes a transaction hash and changes the approval to true for the owner while incrementing the approval count
    fn approveTransaction(tx_hash: b256) -> bool {
        // TODO: signature authentication? ec_recover == signer(s)?
        _is_initialized(storage.state);

        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            _is_owner(get(address.value));

            let tx_data: u64 = get(tx_hash);
            _tx_exists(tx_data);

            let approval_hash = _owner_approval_hash(tx_hash, address);
            let approved: bool = get(approval_hash);

            assert(!approved);

            // TODO: Increment Tx approval if not executed then store it again

            store(approval_hash, true);
            log_b256(address.value);
        } else {
            panic(0);
        };

        true
    }

    /// Takes a transaction hash and changes the approval to false for the owner while decrementing the approval count
    fn revokeTransaction(tx_hash: b256) -> bool {
        // TODO: signature authentication? ec_recover == signer(s)?
        _is_initialized(storage.state);

        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            _is_owner(get(address.value));

            let tx_data: u64 = get(tx_hash);
            _tx_exists(tx_data);

            let approval_hash = _owner_approval_hash(tx_hash, address);
            let approved: bool = get(approval_hash);

            assert(approved);

            // TODO: Decrement Tx approval if not executed then store it again

            store(approval_hash, false);
            log_b256(address.value);
        } else {
            panic(0);
        };

        true
    }

    /// Takes a transaction hash and executes it if the approval hash reached past the threshold
    fn executeTransaction(tx_hash: b256) -> bool {
        _is_initialized(storage.state);

        let sender: Result<Sender, AuthError> = msg_sender();
        if let Sender::Address(address) = sender.unwrap() {
            let tx_data: u64 = get(tx_hash);
            _tx_exists(tx_data);

            // Check approval threshold and if not executed then execute, mark as executed

            log_b256(tx_hash);

            // TODO: execute
        } else {
            panic(0);
        };

        true
    }

    /// Returns a boolean value indicating if the given address is an owner in the contract
    fn is_owner(address: Address) -> bool {
        _is_initialized(storage.state);
        get(address.value)
    }

    /// Returns a boolean value indicating if the given address hash approved the specified transaction
    fn has_approved(tx_hash: b256, owner: Address) -> bool {
        _is_initialized(storage.state);
        let tx_data: u64 = get(tx_hash);
        _tx_exists(tx_data);
        get(_owner_approval_hash(tx_hash, owner))
    }

    ///
    fn get_transaction_hash(to: ContractId, value: u64, data: b256, nonce: u64) -> b256 {
        // TODO: data > b256?
        // TODO: this is probably not following the EIP-191 signing standard. What do?
        let to_hash = hash_value(to.value, HashMethod::Keccak256);
        let data_hash = hash_value(data, HashMethod::Keccak256);
        let value_hash = hash_u64(value, HashMethod::Keccak256);
        let nonce_hash = hash_u64(nonce, HashMethod::Keccak256);

        let id = contract_id();
        hash_pair(id.value, hash_pair(to_hash, hash_pair(value_hash, hash_pair(data_hash, nonce_hash, HashMethod::Keccak256), HashMethod::Keccak256), HashMethod::Keccak256), HashMethod::Keccak256)
    }

    /// Takes a transaction hash and returns the number of approvals and execution state
    fn get_transaction_data(tx_hash: b256) -> (u64, bool) {
        _is_initialized(storage.state);

        let tx_data: u64 = get(tx_hash);
        _tx_exists(tx_data);

        // TODO: extract the approval and execution values

        // return (current approval count, execution state)
        (1, true) // make the compiler happy for now
    }
}

/// Assertion used to ensure that the contract has called the constructor and initialized the values
fn _is_initialized(state: u64) {
    assert(state == 1);
}

/// Assertion used to ensure that the value stored for the owner is true i.e. they are an owner in the contract
fn _is_owner(state: bool) {
    assert(state);
}

/// Returns the hash where the boolean value indicating the approval of the owner is stored
fn _owner_approval_hash(tx_hash: b256, owner: Address) -> b256 {
    hash_pair(owner.value, tx_hash, HashMethod::Sha256)
}

/// Assertion used to ensure that a transaction hash has been submitted to the contract
fn _tx_exists(tx_data: u64) {
    assert(tx_data != 0);
}
