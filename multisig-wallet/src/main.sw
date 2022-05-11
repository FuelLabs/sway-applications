contract;

use std::{address::Address, assert::assert, b512::B512, chain::{auth::{AuthError, Sender, msg_sender}, log_b256}, context::call_frames::contract_id, contract_id::ContractId, ecr::{EcRecoverError, ec_recover_address}, hash::{HashMethod, hash_pair, hash_u64, hash_value}, panic::panic, result::*, storage::{get, store}};

abi MultiSignatureWallet {
    fn constructor(owner1: Address, owner2: Address) -> bool;
    fn submitTransaction(tx_hash: b256) -> bool;
    fn approveTransaction(tx_hash: b256, signatures: [B512;
    2]) -> bool;
    fn revokeTransaction(tx_hash: b256, signatures: [B512;
    2]) -> bool;
    fn executeTransaction(tx_hash: b256) -> bool;
    fn is_owner(owner: Address) -> bool;
    fn has_approved(tx_hash: b256, owner: Address) -> bool;
    fn get_transaction_hash(to: ContractId, value: u64, data: b256, nonce: u64) -> b256;
    fn get_transaction_data(tx_hash: b256) -> (u64, bool);
}

storage {
    /// The state is used as a gatekeeper for unlocking functionality post initialization
    state: u64,
    /// The sentinel is used as a dummy value to store a newly submitted tx hash
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
        // 1111111111111111111111111111111111111111111111111111111111111110
        storage.tx_sentinel = 18446744073709551615 - 1; // Is (2 ^ 64) - 1 equal in sway (question about the ^)?
        true
    }

    /// Takes a transaction hash and stores in it storage
    fn submitTransaction(tx_hash: b256) -> bool {
        _is_initialized(storage.state);

        let sender: Result<Sender, AuthError> = msg_sender();

        if let Sender::Address(address) = sender.unwrap() {
            _is_owner(get(address.value));

            // When a Tx has not been submitted its default value should be 0.
            assert(get::<u64>(tx_hash) == 0);

            // Mark the first submission as a non-zero value
            store(tx_hash, storage.tx_sentinel);

            log_b256(tx_hash);
        } else {
            panic(0);
        };

        true
    }

    /// Takes a transaction hash and changes the approval to true for the owner while incrementing the approval count
    fn approveTransaction(tx_hash: b256, signatures: [B512;
    2]) -> bool {
        _is_initialized(storage.state);

        let tx_data = get::<u64>(tx_hash);
        _tx_exists(tx_data);
        _tx_not_executed(tx_data);

        let mut index = 0;
        // Hardcode signature len
        while index < 2 {
            let signer: Result<Address, EcRecoverError> = ec_recover_address(signatures[index], tx_hash);
            if let Result::Err = signer {
                panic(0);
            };

            let signer = signer.unwrap();
            _is_owner(get(signer.value));

            let approval_hash = _owner_approval_hash(tx_hash, signer);
            let approved = get::<bool>(approval_hash);

            assert(!approved);

            if tx_data == storage.tx_sentinel {
                // Set the first approval by setting it to 2 (first bit is used for execution status)
                store(tx_hash, 2);
            } else if tx_data == storage.tx_sentinel - 2 {
                // Reached conditional limit, cannot increment without looping to condition above after another approval
                panic(0);
            } else {
                // Increment in 2s for approval to keep the execution bit untouched
                store(tx_hash, tx_data + 2);
            }

            // Update owner approval to "has approved"
            store(approval_hash, true);

            log_b256(signer.value);

            index = index + 1;
        };

        true
    }

    /// Takes a transaction hash and changes the approval to false for the owner while decrementing the approval count
    fn revokeTransaction(tx_hash: b256, signatures: [B512;
    2]) -> bool {
        _is_initialized(storage.state);

        let tx_data = get::<u64>(tx_hash);
        _tx_exists(tx_data);
        _tx_not_executed(tx_data);

        let mut index = 0;
        // Hardcode signature len
        while index < 2 {
            let signer: Result<Address, EcRecoverError> = ec_recover_address(signatures[index], tx_hash);
            if let Result::Err = signer {
                panic(0);
            };

            let signer = signer.unwrap();
            _is_owner(get(signer.value));

            let approval_hash = _owner_approval_hash(tx_hash, signer);
            let approved = get::<bool>(approval_hash);

            assert(approved);

            if tx_data == 2 {
                // Reset back to original "zero" value i.e. the sentinel
                store(tx_hash, storage.tx_sentinel);
            } else {
                // Decrement in 2s for approval to keep the execution bit untouched
                store(tx_hash, tx_data - 2);
            }

            // Update owner approval to "has not approved"
            store(approval_hash, false);

            log_b256(signer.value);

            index = index + 1;
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
            _tx_not_executed(tx_data);

            // TODO: Check approval threshold: https://github.com/FuelLabs/sway-applications/issues/3

            // Mark as executed
            store(tx_hash, tx_data + 1);

            log_b256(tx_hash);

            // TODO: https://github.com/FuelLabs/sway-applications/issues/6 and/or https://github.com/FuelLabs/sway-applications/issues/22
            // execute
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
        _tx_exists(get::<u64>(tx_hash));
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

        let tx_data = get::<u64>(tx_hash);
        _tx_exists(tx_data);

        let executed = tx_data % 2 == 1;
        let mut approvals = 0;
        if executed {
            if tx_data != storage.tx_sentinel + 1 {
                approvals = (tx_data - 1) / 2;
            };
        } else {
            if tx_data != storage.tx_sentinel {
                approvals = tx_data / 2;
            };
        };

        (approvals, executed)
    }
}

/// Assertion used to ensure that the value stored for the owner is true i.e. they are an owner in the contract
fn _is_owner(state: bool) {
    assert(state);
}

/// Assertion used to make sure that the last bit is still 0 i.e. has not been executed
fn _tx_not_executed(tx_data: u64) {
    assert(tx_data % 2 == 0);
}

/// Assertion used to ensure that the contract has called the constructor and initialized the values
fn _is_initialized(state: u64) {
    assert(state == 1);
}

/// Returns the hash where the boolean value indicating the approval of the owner is stored
fn _owner_approval_hash(tx_hash: b256, owner: Address) -> b256 {
    hash_pair(owner.value, tx_hash, HashMethod::Sha256)
}

/// Assertion used to ensure that a transaction hash has been submitted to the contract
fn _tx_exists(tx_data: u64) {
    assert(tx_data != 0);
}
