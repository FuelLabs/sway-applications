
contract;

// TODO: commented out code is because the SDK does not support Vec yet

// Our library dependencies
dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

// Standard library code
use std::{
    address::Address,
    b512::B512,
    constants::ZERO_B256,
    context::{call_frames::contract_id, this_balance},
    contract_id::ContractId,
    ecr::ec_recover_address,
    identity::Identity,
    logging::log,
    result::Result,
    revert::{require, revert},
    storage::StorageMap,
    token::transfer,
    // vec::Vec,
};

// Our library imports
use data_structures::{Owner, User};
use errors::{ExecutionError, InitError};
use events::{ExecutedEvent, TransferEvent};
use interface::MultiSignatureWallet;
use utils::create_hash;

storage {
    /// Used to add entropy into hashing of Tx to decrease the probability of collisions / double
    /// spending
    nonce: u64 = 0,

    /// The number of approvals required in order to execture a Tx
    threshold: u64 = 0,

    /// Number of approvals per user
    weighting: StorageMap<Address, Owner> = StorageMap {}, 
}

impl MultiSignatureWallet for Contract {
    fn balance(asset_id: ContractId) -> u64 {
        this_balance(asset_id)
    }

    // #[storage(read, write)]fn constructor(threshold: u64, users: Vec<User>) {
    #[storage(read, write)]fn constructor(threshold: u64, users: [User; 3]) {
        require(storage.nonce == 0, InitError::CannotReinitialize);
        require(threshold != 0, InitError::ThresholdCannotBeZero);

        let mut user_index = 0;
        while user_index < 3 {
            let user = users[user_index];
        // while user_index < users.len() {
            // let user = users.get(user_index).unwrap();

            require(user.weight != 0, InitError::WeightingCannotBeZero);

            storage.weighting.insert(user.identity, Owner {
                weight: user.weight
            });
            user_index += 1;
        }

        storage.nonce = 1;
        storage.threshold = threshold;
    }

    // #[storage(read, write)]fn execute_transaction(to: Identity, value: u64, data: Vec<u64>, signatures: Vec<B512>) {
    #[storage(read, write)]fn execute_transaction(to: Identity, value: u64, data: [u64; 3], signatures: [B512; 3]) {
        require(storage.nonce != 0, InitError::NotInitialized);

        let transaction_hash = create_hash(to, value, data, storage.nonce, contract_id());
        let approval_count = count_approvals(transaction_hash, signatures);

        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce += 1;

        // TODO: Execute https://github.com/FuelLabs/sway-applications/issues/22

        log(ExecutedEvent {
            to, value, data, nonce: storage.nonce - 1
        });
    }

    #[storage(read)]fn nonce() -> u64 {
        storage.nonce
    }

    #[storage(read)]fn owner(user: Address) -> Owner {
        storage.weighting.get(user)
    }

    #[storage(read, write)]fn transfer(to: Identity, asset_id: ContractId, value: u64, data: [u64; 3], signatures: [B512; 3]) {
        require(storage.nonce != 0, InitError::NotInitialized);
        require(value <= this_balance(asset_id), ExecutionError::InsufficientAssetAmount);

        let transaction_hash = create_hash(to, value, data, storage.nonce, contract_id());
        let approval_count = count_approvals(transaction_hash, signatures);

        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce += 1;

        transfer(value, asset_id, to);

        log(TransferEvent {
            to, asset: asset_id, value, nonce: storage.nonce - 1
        });
    }
    
    fn transaction_hash(to: Identity, value: u64, data: [u64; 3], nonce: u64) -> b256 {
        create_hash(to, value, data, nonce, contract_id())
    }
}

// Unfortunately, this requires storage access and there's no way for a library to access it
// There's no good way to re-write this into utils.sw. At best we can duplicate the loop which is bad
#[storage(read)]fn count_approvals(transaction_hash: b256, signatures: [B512; 3]) -> u64 {
    // The signers must have increasing values in order to check for duplicates or a zero-value
    let mut previous_signer = ZERO_B256;

    let mut approval_count = 0;
    let mut index = 0;
    // while index < signatures.len() {
    while index < 3 {
        // let signature = signatures.get(index);
        let signature = signatures[index];
        let signer = match ec_recover_address(signature, transaction_hash) {
            Result::Ok(address) => address.value, _ => revert(42), 
        };

        require(previous_signer < signer, ExecutionError::IncorrectSignerOrdering);

        previous_signer = signer;
        approval_count += storage.weighting.get(~Address::from(signer)).weight;

        if storage.threshold <= approval_count {
            break;
        }

        index += 1;
    }

    approval_count
}
