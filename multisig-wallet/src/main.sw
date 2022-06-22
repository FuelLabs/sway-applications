contract;

// Our library dependencies
dep abi;
dep data_structures;
dep errors;
dep events;

// Standard library code
use std::{
    address::Address,
    assert::require,
    b512::B512,
    constants::BASE_ASSET_ID,
    context::{call_frames::contract_id, this_balance},
    contract_id::ContractId,
    ecr::ec_recover_address,
    hash::sha256,
    identity::Identity,
    logging::log,
    option::*,
    result::Result,
    revert::revert,
    storage::StorageMap,
    token::transfer,
    vec::Vec,
};

use core::num::*;

// Our library imports
use abi::MultiSignatureWallet;
use data_structures::{Owner, Transaction, User};
use errors::{ExecutionError, InitError};
use events::{ExecutedEvent, TransferEvent};

storage {
    /// Used to add entropy into hashing of Tx to decrease the probability of collisions / double
    /// spending
    nonce: u64,

    /// The number of approvals required in order to execture a Tx
    threshold: u64,

    /// Number of approvals per user
    weighting: StorageMap<Address,
    Owner>, 
}

impl MultiSignatureWallet for Contract {
    /// The constructor initializes the necessary values and unlocks further functionlity
    ///
    /// # Arguments
    ///
    /// - `users` -
    /// - `threshold` -
    ///
    /// # Reverts
    ///
    /// - When the constructor is called more than once
    /// - When the user address is the 0th address (0x00000...)
    /// - When the threshold is set to 0
    /// - When an owner has an approval weight of 0
    #[storage(read, write)]fn constructor(users: Vec<User>, threshold: u64) {
        require(storage.nonce == 0, InitError::CannotReinitialize);
        require(storage.threshold != 0, InitError::ThresholdCannotBeZero);

        let mut user_index = 0;
        while user_index < users.len() {
            // Workaround
            let user: Option<User> = users.get(user_index);
            let user = user.unwrap();

            require(BASE_ASSET_ID != user.identity.value, InitError::AddressCannotBeZero);
            require(user.weight != 0, InitError::WeightingCannotBeZero);

            storage.weighting.insert(user.identity, Owner {
                exists: true, weight: user.weight
            });
            user_index = user_index + 1;
        }

        storage.nonce = 1;
        storage.threshold = threshold;
    }

    /// Executes a Tx formed from the `to, `value` and `data` parameters if the signatures meet the
    /// threshold requirement
    ///
    /// # Arguments
    ///
    /// - `to` -
    /// - `value` -
    /// - `data` -
    /// - `signatures` -
    ///
    /// # Reverts
    ///
    /// - When the constructor has not been called to initialize the contract
    /// - When the public key cannot be recovered from a signature
    /// - When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...)
    /// - When the total approval count is less than the required threshold for execution
    #[storage(read, write)]fn execute_transaction(to: Identity, value: u64, data: Vec<u64>, signatures: Vec<B512>) {
        require(storage.nonce != 0, InitError::NotInitialized);

        let transaction_hash = create_hash(to, value, data, storage.nonce, contract_id());
        let approval_count = count_approvals(transaction_hash, signatures);

        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce = storage.nonce + 1;

        // TODO: Execute https://github.com/FuelLabs/sway-applications/issues/22

        log(ExecutedEvent {
            to, value, data, nonce: storage.nonce - 1
        });
    }

    /// Transfers assets to outputs & contracts if the signatures meet the threshold requirement
    ///
    /// # Arguments
    ///
    /// - `to` -
    /// - `value` -
    /// - `data` -
    /// - `asset_id` -
    /// - `signatures` -
    ///
    /// # Reverts
    ///
    /// - When the constructor has not been called to initialize the contract
    /// - When the amount of the asset being sent is greater than the balance in the contract
    /// - When the public key cannot be recovered from a signature
    /// - When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...)
    /// - When the total approval count is less than the required threshold for execution
    #[storage(read, write)]fn transfer(to: Identity, asset_id: ContractId, value: u64, data: Vec<u64>, signatures: Vec<B512>) {
        require(storage.nonce != 0, InitError::NotInitialized);
        require(value <= this_balance(asset_id), ExecutionError::InsufficientAssetAmount);

        let transaction_hash = create_hash(to, value, data, storage.nonce, contract_id());
        let approval_count = count_approvals(transaction_hash, signatures);

        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce = storage.nonce + 1;

        transfer(value, asset_id, to);

        log(TransferEvent {
            to, asset: asset_id, value, nonce: storage.nonce - 1
        });
    }

    /// Returns an Owner struct indicating whether the user is an owner and their approval weight
    ///
    /// # Arguments
    ///
    /// - `user` -
    #[storage(read)]fn owner(user: Address) -> Owner {
        storage.weighting.get(user)
    }

    /// Returns the current nonce in the contract
    /// Used to check the nonce and create a Tx via transaction_hash()
    #[storage(read)]fn nonce() -> u64 {
        storage.nonce
    }

    /// Returns the balance of the specified asset_id for this contract
    ///
    /// # Arguments
    ///
    /// - `asset_id` -
    fn balance(asset_id: ContractId) -> u64 {
        this_balance(asset_id)
    }

    /// Takes in transaction data and hashes it into a unique tx hash
    /// Used for verification of message
    ///
    /// # Arguments
    ///
    /// - `to` -
    /// - `value` -
    /// - `data` -
    /// - `nonce` -
    fn transaction_hash(to: Identity, value: u64, data: Vec<u64>, nonce: u64) -> b256 {
        create_hash(to, value, data, nonce, contract_id())
    }
}

fn create_hash(to: Identity, value: u64, data: Vec<u64>, nonce: u64, self_id: ContractId) -> b256 {
    sha256(Transaction {
        contract_identifier: self_id, destination: to, value, data, nonce
    })
}

#[storage(read)]fn count_approvals(transaction_hash: b256, signatures: Vec<B512>) -> u64 {
    // The signers must have increasing values in order to check for duplicates or a zero-value
    let mut previous_signer = ~b256::min();

    let mut approval_count = 0;
    let mut index = 0;
    while index < signatures.len() {
        let signature: Option<B512> = signatures.get(index);
        let signer = match ec_recover_address(signature.unwrap(), transaction_hash) {
            Result::Ok(address) => address.value, _ => revert(42), 
        };

        require(previous_signer < signer, ExecutionError::IncorrectSignerOrdering);

        previous_signer = signer;
        approval_count = approval_count + storage.weighting.get(~Address::from(signer)).weight;

        // Once break is implemented uncomment below. https://github.com/FuelLabs/sway/pull/1646
        // if storage.threshold <= approval_count {
        //     break;
        // }

        index = index + 1;
    }

    approval_count
}
