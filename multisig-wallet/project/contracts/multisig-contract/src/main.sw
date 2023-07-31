contract;

// TODO:
//      - change the "data" in the Tx hashing from b256 to Bytes type when SDK support is implemented: https://github.com/FuelLabs/fuels-rs/issues/723.
//    
mod data_structures;
mod errors;
mod events;
mod interface;
mod utils;

use ::errors::{ExecutionError, InitError};
use ::events::{ExecuteTransactionEvent, SetThresholdEvent, SetWeightEvent};
use ::interface::{Info, MultiSignatureWallet};
use ::data_structures::{
    hashing::{
        ContractCallParams,
        Threshold,
        Transaction,
        TransferParams,
        TypeToHash,
        Weight,
    },
    signatures::SignatureInfo,
    user::User,
};
use std::{
    auth::msg_sender,
    call_frames::contract_id,
    context::this_balance,
    error_signals::FAILED_REQUIRE_SIGNAL,
    hash::sha256,
    low_level_call::{
        call_with_function_selector,
        CallParams,
    },
    token::transfer,
};
use ::utils::recover_signer;

configurable {
    THRESHOLD: u64 = 5,
}

storage {
    /// Used to add entropy into hashing of transaction to decrease the probability of collisions / double
    /// spending.
    nonce: u64 = 0,
    /// The total weight of all the user approvals
    total_weight: u64 = 0,
    /// The number of approvals required in order to execute a transaction.
    threshold: u64 = 0,
    /// Number of approvals per user.
    weighting: StorageMap<b256, u64> = StorageMap {},
}

impl MultiSignatureWallet for Contract {
    #[storage(read, write)]
    fn constructor(users: Vec<User>) {
        require(storage.nonce.read() == 0, InitError::CannotReinitialize);
        require(THRESHOLD != 0, InitError::ThresholdCannotBeZero);

        let mut user_index = 0;
        let mut total_weight = 0;
        while user_index < users.len() {
            storage.weighting.insert(users.get(user_index).unwrap().address, users.get(user_index).unwrap().weight);
            total_weight += users.get(user_index).unwrap().weight;

            user_index += 1;
        }

        require(THRESHOLD <= total_weight, InitError::TotalWeightCannotBeLessThanThreshold);

        storage.nonce.write(1);
        storage.threshold.write(THRESHOLD);
        storage.total_weight.write(total_weight);
    }

    #[storage(read, write)]
    fn set_threshold(signatures: Vec<SignatureInfo>, threshold: u64) {
        let nonce = storage.nonce.read();
        require(nonce != 0, InitError::NotInitialized);
        require(threshold != 0, InitError::ThresholdCannotBeZero);
        require(threshold <= storage.total_weight.read(), InitError::TotalWeightCannotBeLessThanThreshold);

        let transaction_hash = compute_hash(TypeToHash::Threshold(Threshold {
            contract_identifier: contract_id(),
            nonce,
            threshold,
        }));
        let approval_count = count_approvals(signatures, transaction_hash);

        let previous_threshold = storage.threshold.read();
        require(previous_threshold <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce.write(nonce + 1);
        storage.threshold.write(threshold);

        log(SetThresholdEvent {
            nonce,
            previous_threshold,
            threshold,
        });
    }

    #[storage(read, write)]
    fn set_weight(signatures: Vec<SignatureInfo>, user: User) {
        let nonce = storage.nonce.read();
        require(nonce != 0, InitError::NotInitialized);

        let transaction_hash = compute_hash(TypeToHash::Weight(Weight {
            contract_identifier: contract_id(),
            nonce,
            user,
        }));
        let approval_count = count_approvals(signatures, transaction_hash);

        let threshold = storage.threshold.read();
        require(threshold <= approval_count, ExecutionError::InsufficientApprovals);

        let current_weight = storage.weighting.get(user.address).try_read().unwrap_or(0);

        let total_weight = storage.total_weight.read();
        if current_weight < user.weight {
            storage.total_weight.write(total_weight + (user.weight - current_weight));
        } else if user.weight < current_weight {
            storage.total_weight.write(total_weight - (current_weight - user.weight));
        }

        require(threshold <= total_weight, InitError::TotalWeightCannotBeLessThanThreshold);

        storage.weighting.insert(user.address, user.weight);
        storage.nonce.write(nonce + 1);

        log(SetWeightEvent { nonce, user })
    }

    #[storage(read, write)]
    fn transfer(
        asset_id: ContractId,
        data: b256,
        signatures: Vec<SignatureInfo>,
        to: Identity,
        value: u64,
    ) {
        require(storage.nonce.read() != 0, InitError::NotInitialized);
        require(value <= this_balance(asset_id), ExecutionError::InsufficientAssetAmount);

        let transaction_hash = hash_transaction(data, storage.nonce.read(), to, value);
        let approval_count = count_approvals(signatures, transaction_hash);
        require(storage.threshold.read() <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce.write(storage.nonce.read() + 1);

        transfer(value, asset_id, to);

        log(TransferEvent {
            asset: asset_id,
            nonce: storage.nonce.read() - 1,
            to,
            value,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn approval_weight(user: b256) -> u64 {
        storage.weighting.get(user).try_read().unwrap_or(0)
    }

    fn balance(asset_id: ContractId) -> u64 {
        this_balance(asset_id)
    }

    fn compute_hash(type_to_hash: TypeToHash) -> b256 {
        compute_hash(type_to_hash)
    }

    #[storage(read)]
    fn nonce() -> u64 {
        storage.nonce.read()
    }

    #[storage(read)]
    fn threshold() -> u64 {
        storage.threshold.read()
    }
    }

fn compute_hash(type_to_hash: TypeToHash) -> b256 {
    match type_to_hash {
        TypeToHash::Threshold(threshold) => sha256(threshold),
        TypeToHash::Transaction(transaction) => transaction.into_bytes().sha256(),
        TypeToHash::Weight(weight) => sha256(weight),
    }
}

/// Takes in a transaction hash and signatures with associated data.
/// Recovers a b256 address from each signature;
/// it then increments the number of approvals by that address' approval weighting.
/// Returns the final approval count.
#[inline(never)]
#[storage(read)]
fn count_approvals(signatures: Vec<SignatureInfo>, transaction_hash: b256) -> u64 {
    // The signers must have increasing values in order to check for duplicates or a zero-value.
    let mut previous_signer = b256::min();

    let mut approval_count = 0;
    let mut index = 0;
    while index < signatures.len() {
        let signer = recover_signer(transaction_hash, signatures.get(index).unwrap());

        require(previous_signer < signer, ExecutionError::IncorrectSignerOrdering);

        previous_signer = signer;
        approval_count += storage.weighting.get(signer).try_read().unwrap_or(0);

        if storage.threshold.read() <= approval_count {
            break;
        }

        index += 1;
    }
    approval_count
}
