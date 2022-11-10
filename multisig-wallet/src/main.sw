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
    call_frames::contract_id,
    constants::ZERO_B256,
    context::this_balance,
    contract_id::ContractId,
    ecr::ec_recover_address,
    hash::sha256,
    identity::Identity,
    logging::log,
    result::*,
    revert::{
        require,
        revert,
    },
    storage::StorageMap,
    token::{
        force_transfer_to_contract,
        transfer_to_address,
    },
};

use core::num::*;

// Our library imports
use contract_abi::MultiSignatureWallet;
use data_structures::{SignatureData, Transaction, User};
use errors::{ExecutionError, InitError};
use events::{ExecutedEvent, TransferEvent};

storage {
    /// Used to add entropy into hashing of Tx to decrease the probability of collisions / double
    /// spending
    nonce: u64 = 0,
    /// The number of approvals required in order to execture a Tx
    threshold: u64 = 0,
    /// Number of approvals per user
    weighting: StorageMap<b256, u64> = StorageMap {},
}

impl MultiSignatureWallet for Contract {
    #[storage(read, write)]
    fn constructor(users: Vec<User>, threshold: u64) {
        require(storage.nonce == 0, InitError::CannotReinitialize);
        require(threshold != 0, InitError::ThresholdCannotBeZero);

        let mut user_index = 0;
        while user_index < users.len() {
            require(ZERO_B256 != users.get(user_index).unwrap().identity, InitError::AddressCannotBeZero);
            require(users.get(user_index).unwrap().weight != 0, InitError::WeightingCannotBeZero);

            storage.weighting.insert(users.get(user_index).unwrap().identity, users.get(user_index).unwrap().weight);
            user_index += 1;
        }

        storage.nonce = 1;
        storage.threshold = threshold;
    }

    #[storage(read, write)]
    fn execute_transaction(
        to: Identity,
        value: u64,
        data: b256,
        signatures_data: Vec<SignatureData>,
    ) {
        require(storage.nonce != 0, InitError::NotInitialized);

        let transaction_hash = create_hash(to, value, data, storage.nonce, contract_id());
        let approval_count = count_approvals(transaction_hash, signatures_data);

        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce += 1;

        // TODO: Execute https://github.com/FuelLabs/sway-applications/issues/22
        log(ExecutedEvent {
            to,
            value,
            data,
            nonce: storage.nonce - 1,
        });
    }

    #[storage(read, write)]
    fn transfer(
        to: Identity,
        asset_id: ContractId,
        value: u64,
        data: b256,
        signatures_data: Vec<SignatureData>,
    ) {
        require(storage.nonce != 0, InitError::NotInitialized);
        require(value <= this_balance(asset_id), ExecutionError::InsufficientAssetAmount);

        let transaction_hash = create_hash(to, value, data, storage.nonce, contract_id());
        let approval_count = count_approvals(transaction_hash, signatures_data);

        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce += 1;

        match to {
            Identity::Address(address) => transfer_to_address(value, asset_id, address),
            Identity::ContractId(address) => force_transfer_to_contract(value, asset_id, address),
        };

        log(TransferEvent {
            to,
            asset: asset_id,
            value,
            nonce: storage.nonce - 1,
        });
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
    #[storage(read)]
    fn nonce() -> u64 {
        storage.nonce
    }
}

fn create_hash(
    to: Identity,
    value: u64,
    data: b256,
    nonce: u64,
    self_id: ContractId,
) -> b256 {
    sha256(Transaction {
        contract_identifier: self_id,
        destination: to,
        value,
        data,
        nonce,
    })
}

#[storage(read)]
fn count_approvals(transaction_hash: b256, signatures: [B512; 25]) -> u64 {
    // The signers must have increasing values in order to check for duplicates or a zero-value
    let mut previous_signer = b256::min();

    let mut approval_count = 0;
    let mut index = 0;
    while index < 25 {
        let signer = match ec_recover_address(signatures[index], transaction_hash) {
            Result::Ok(address) => address.value,
            _ => revert(42),
        };

        require(previous_signer < signer, ExecutionError::IncorrectSignerOrdering);

        previous_signer = signer;
        approval_count = approval_count + storage.weighting.get(Address::from(signer));

        // Once break is implemented uncomment below. https://github.com/FuelLabs/sway/pull/1646
        // if storage.threshold <= approval_count {
        //     break;
        // }
        index = index + 1;
    }

    approval_count
}
