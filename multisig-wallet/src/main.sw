contract;

use std::{
    address::Address,
    assert::require,
    b512::B512,
    context::call_frames::contract_id,
    contract_id::ContractId,
    ecr::{EcRecoverError, ec_recover_address},
    hash::sha256,
    logging::log,
    result::*,
    revert::revert,
    storage::{get, store}
};

use core::num::*;

abi MultiSignatureWallet {
    fn constructor(owner1: Address, owner2: Address, owner1_weight: u64, owner2_weight: u64, threshold: u64) -> bool;
    fn execute_transaction(to: ContractId, value: u64, data: b256, signature1: B512, signature2: B512) -> bool;
    fn is_owner(owner: Address) -> bool;
    fn get_transaction_hash(to: ContractId, value: u64, data: b256, nonce: u64) -> b256;
}

enum Error {
    ApprovalThresholdNotReached: (),
    CannotReinitialize: (),
    IncorrectSignerOrdering: (),
    NotAnOwner: (),
    NotInitialized: (),
    ThresholdCannotBeZero: (),
    WeightingCannotBeZero: ()
}

struct Tx {
    destination: b256,
    value: u64,
    data: b256,
    nonce: u64,
    contract_identifier: b256,
}

struct User {
    address: Address,
    weight: u64
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
    /// # Panics
    ///
    /// - When the constructor is called more than once
    /// - When the threshold is set to 0
    /// - When an owner has an approval weight of 0
    fn constructor(owner1: Address, owner2: Address, owner1_weight: u64, owner2_weight: u64, threshold: u64) -> bool {
        require(storage.nonce == 0, Error::CannotReinitialize);
        require(storage.threshold != 0, Error::ThresholdCannotBeZero);
        
        // TODO: when vectors are implemented change owners to be a Vec<Address>
        require(owner1_weight != 0, Error::WeightingCannotBeZero);
        require(owner2_weight != 0, Error::WeightingCannotBeZero);

        store(owner1.value, owner1_weight);
        store(owner2.value, owner2_weight);

        storage.nonce = 1;
        storage.threshold = threshold;
        true
    }

    /// Executes a Tx if the required signatures meet the restrictions on ownership and threshold approval
    ///
    /// # Panics
    ///
    /// - When the constructor has not been called to initialize the contract
    /// - When the public key cannot be recovered from a signature
    /// - When the signer is not an owner
    /// - When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...)
    /// - When the total approval count is less than the required threshold for execution
    fn execute_transaction(to: ContractId, value: u64, data: b256, signature1: B512, signature2: B512) -> bool {
        require(storage.nonce != 0, Error::NotInitialized);

        let tx_hash = _get_transaction_hash(to, value, data, storage.nonce, contract_id());

        // TODO: change to Vec<B512> once implemented and then iterate instead of hardcoding length
        let signer1: b256 = match ec_recover_address(signature1, tx_hash) {
            Result::Ok(address) => address.value, _ => revert(42), 
        };

        let signer2: b256 = match ec_recover_address(signature2, tx_hash) {
            Result::Ok(address) => address.value, _ => revert(42), 
        };

        require(~b256::min() < signer1 && signer1 < signer2, Error::IncorrectSignerOrdering);

        let signer1_weight = get::<u64>(signer1);
        let signer2_weight = get::<u64>(signer2);

        require(signer1_weight != 0 && signer2_weight != 0, Error::NotAnOwner);

        // Hardcoded value, that passes the checks above, until the loop below is unblocked
        let approval_count = signer1_weight + signer2_weight;

        // The signers must have increasing values in order to check for duplicates or a zero-value
        // let mut previous_signer: b256 = ~b256::min();

        // let mut approval_count = 0;
        // let mut index = 0;
        // while index < 2 {
        //     let signer: b256 = match ec_recover_address(signatures[index], tx_hash) {
        //         Result::Ok(address) => address.value, _ => revert(42), 
        //     };

        //     require(previous_signer < signer, Error::IncorrectSignerOrdering);

        //     let signer_weight = get::<u64>(signer);
        //     require(signer_weight != 0, Error::NotAnOwner);

        //     previous_signer = signer;
        //     approval_count = approval_count + signer_weight;

        //     // Once break is implemented uncomment below. https://github.com/FuelLabs/sway/pull/1646
        //     // if storage.threshold <= approval_count {
        //     //     break;
        //     // }

        //     index = index + 1;
        // }

        require(storage.threshold <= approval_count, Error::ApprovalThresholdNotReached);

        storage.nonce = storage.nonce + 1;
        log(tx_hash);

        // TODO: Execute (https://github.com/FuelLabs/sway-applications/issues/6 and/or https://github.com/FuelLabs/sway-applications/issues/22)

        true
    }

    /// Takes in transaction data and hashes it into a unique tx hash
    /// Used for verification of message
    fn get_transaction_hash(to: ContractId, value: u64, data: b256, nonce: u64) -> b256 {
        // TODO: data > b256?
        _get_transaction_hash(to, value, data, nonce, contract_id())
    }

    /// Returns a boolean value indicating if the given address is an owner in the contract
    ///
    /// # Panics
    ///
    /// - When the constructor has not been called to initialize the contract
    fn is_owner(address: Address) -> bool {
        require(storage.nonce != 0, Error::NotInitialized);
        get::<u64>(address.value) != 0
    }
}

fn _get_transaction_hash(to: ContractId, value: u64, data: b256, nonce: u64, self_id: ContractId) -> b256 {
    sha256(Tx {
        contract_identifier: self_id.value, destination: to.value, value, data, nonce
    })
}
