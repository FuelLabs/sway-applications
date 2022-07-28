contract;

// TODO: when vectors are implemented
//      - change all uses of arrays to vec
//      - change the "data" in the Tx hashing from b256 to vec

// Our library dependencies
dep contract_abi;
dep data_structures;
dep errors;
dep events;

// Standard library code
use std::{
    address::Address,
    b512::B512,
    constants::ZERO_B256,
    context::{call_frames::contract_id, this_balance},
    contract_id::ContractId,
    ecr::ec_recover_address,
    hash::sha256,
    identity::Identity,
    logging::log,
    result::*,
    revert::{require, revert},
    storage::StorageMap,
    token::{force_transfer_to_contract, transfer_to_output}
};

use core::num::*;

// Our library imports
use contract_abi::MultiSignatureWallet;
use data_structures::{Transaction, User};
use errors::{ExecutionError, InitError};
use events::{ExecutedEvent, TransferEvent};

storage {
    /// Used to add entropy into hashing of Tx to decrease the probability of collisions / double
    /// spending
    nonce: u64 = 0,

    /// The number of approvals required in order to execture a Tx
    threshold: u64 = 0,

    /// Number of approvals per user
    weighting: StorageMap<Address,
    u64> = StorageMap {
    },
}

impl MultiSignatureWallet for Contract {
    /// The constructor initializes the necessary values and unlocks further functionlity
    ///
    /// # Panics
    ///
    /// - When the constructor is called more than once
    /// - When the user address is the 0th address (0x00000...)
    /// - When the threshold is set to 0
    /// - When an owner has an approval weight of 0
    #[storage(read, write)]fn constructor(users: [User;
    25], threshold: u64) {
        require(storage.nonce == 0, InitError::CannotReinitialize);
        require(storage.threshold != 0, InitError::ThresholdCannotBeZero);

        let mut user_index = 0;
        while user_index < 25 {
            require(~Address::from(ZERO_B256) != users[user_index].identity, InitError::AddressCannotBeZero);
            require(users[user_index].weight != 0, InitError::WeightingCannotBeZero);
            storage.weighting.insert(users[user_index].identity, users[user_index].weight);
            user_index = user_index + 1;
        }

        storage.nonce = 1;
        storage.threshold = threshold;
    }

    /// Executes a Tx formed from the `to, `value` and `data` parameters if the signatures meet the
    /// threshold requirement
    ///
    /// # Panics
    ///
    /// - When the constructor has not been called to initialize the contract
    /// - When the public key cannot be recovered from a signature
    /// - When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...)
    /// - When the total approval count is less than the required threshold for execution
    #[storage(read, write)]fn execute_transaction(to: Identity, value: u64, data: b256, signatures: [B512;
    25]) {
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
    /// # Panics
    ///
    /// - When the constructor has not been called to initialize the contract
    /// - When the amount of the asset being sent is greater than the balance in the contract
    /// - When the public key cannot be recovered from a signature
    /// - When the recovered addresses are not in ascending order (0x1 < 0x2 < 0x3...)
    /// - When the total approval count is less than the required threshold for execution
    #[storage(read, write)]fn transfer(to: Identity, asset_id: ContractId, value: u64, data: b256, signatures: [B512;
    25]) {
        require(storage.nonce != 0, InitError::NotInitialized);
        require(value <= this_balance(asset_id), ExecutionError::InsufficientAssetAmount);

        let transaction_hash = create_hash(to, value, data, storage.nonce, contract_id());
        let approval_count = count_approvals(transaction_hash, signatures);

        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce = storage.nonce + 1;

        match to {
            Identity::Address(address) => transfer_to_output(value, asset_id, address), Identity::ContractId(address) => force_transfer_to_contract(value, asset_id, address), 
        };

        log(TransferEvent {
            to, asset: asset_id, value, nonce: storage.nonce - 1
        });
    }

    /// Returns a boolean value indicating if the given address is a user in the contract
    ///
    /// # Panics
    ///
    /// - When the constructor has not been called to initialize the contract
    #[storage(read)]fn is_owner(user: Address) -> bool {
        require(storage.nonce != 0, InitError::NotInitialized);
        storage.weighting.get(user) != 0
    }

    /// Returns the balance of the specified asset_id for this contract
    fn balance(asset_id: ContractId) -> u64 {
        this_balance(asset_id)
    }

    /// Takes in transaction data and hashes it into a unique tx hash
    /// Used for verification of message
    fn transaction_hash(to: Identity, value: u64, data: b256, nonce: u64) -> b256 {
        create_hash(to, value, data, nonce, contract_id())
    }

    /// Returns the current nonce in the contract
    /// Used to check the nonce and create a Tx via transaction_hash()
    #[storage(read)]fn nonce() -> u64 {
        storage.nonce
    }
}

fn create_hash(to: Identity, value: u64, data: b256, nonce: u64, self_id: ContractId) -> b256 {
    sha256(Transaction {
        contract_identifier: self_id, destination: to, value, data, nonce
    })
}

#[storage(read)]fn count_approvals(transaction_hash: b256, signatures: [B512;
25]) -> u64 {
    // The signers must have increasing values in order to check for duplicates or a zero-value
    let mut previous_signer = ~b256::min();

    let mut approval_count = 0;
    let mut index = 0;
    while index < 25 {
        let signer = match ec_recover_address(signatures[index], transaction_hash) {
            Result::Ok(address) => address.value, _ => revert(42), 
        };

        require(previous_signer < signer, ExecutionError::IncorrectSignerOrdering);

        previous_signer = signer;
        approval_count = approval_count + storage.weighting.get(~Address::from(signer));

        // Once break is implemented uncomment below. https://github.com/FuelLabs/sway/pull/1646
        // if storage.threshold <= approval_count {
        //     break;
        // }

        index = index + 1;
    }

    approval_count
}
