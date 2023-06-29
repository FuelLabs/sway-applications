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
use ::events::{ExecutedEvent, SetThresholdEvent, SetWeightEvent, TransferEvent};
use ::interface::{Info, MultiSignatureWallet};
use ::data_structures::{signatures::SignatureInfo, user::User};
use std::{auth::msg_sender, context::this_balance, token::transfer};
use ::utils::{hash_threshold, hash_transaction, hash_weight, recover_signer};

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
        require(storage.nonce == 0, InitError::CannotReinitialize);
        require(THRESHOLD != 0, InitError::ThresholdCannotBeZero);

        let mut user_index = 0;
        let mut total_weight = 0;
        while user_index < users.len() {
            storage.weighting.insert(users.get(user_index).unwrap().address, users.get(user_index).unwrap().weight);
            total_weight += users.get(user_index).unwrap().weight;

            user_index += 1;
        }

        require(THRESHOLD <= total_weight, InitError::TotalWeightCannotBeLessThanThreshold);

        storage.nonce = 1;
        storage.threshold = THRESHOLD;
        storage.total_weight = total_weight;
    }

    #[storage(read, write)]
    fn execute_transaction(
        data: b256,
        signatures: Vec<SignatureInfo>,
        to: Identity,
        value: u64,
    ) {
        require(storage.nonce != 0, InitError::NotInitialized);

        let transaction_hash = hash_transaction(data, storage.nonce, to, value);
        let approval_count = count_approvals(signatures, transaction_hash);

        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce += 1;

        // TODO: Execute https://github.com/FuelLabs/sway-applications/issues/22
        log(ExecutedEvent {
            data,
            nonce: storage.nonce - 1,
            to,
            value,
        });
    }

    #[storage(read, write)]
    fn set_threshold(
        data: Option<b256>,
        signatures: Vec<SignatureInfo>,
        threshold: u64,
    ) {
        require(storage.nonce != 0, InitError::NotInitialized);
        require(threshold != 0, InitError::ThresholdCannotBeZero);
        require(threshold <= storage.total_weight, InitError::TotalWeightCannotBeLessThanThreshold);

        let transaction_hash = hash_threshold(data, storage.nonce, threshold);
        let approval_count = count_approvals(signatures, transaction_hash);

        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        let previous_threshold = storage.threshold;

        storage.nonce += 1;
        storage.threshold = threshold;

        log(SetThresholdEvent {
            previous_threshold,
            threshold,
        });
    }

    #[storage(read, write)]
    fn set_weight(
        data: Option<b256>,
        signatures: Vec<SignatureInfo>,
        user: User,
    ) {
        require(storage.nonce != 0, InitError::NotInitialized);

        let transaction_hash = hash_weight(data, storage.nonce, user);
        let approval_count = count_approvals(signatures, transaction_hash);

        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        let current_weight = storage.weighting.get(user.address).unwrap_or(0);

        if current_weight < user.weight {
            storage.total_weight += user.weight - current_weight;
        } else if user.weight < current_weight {
            storage.total_weight -= current_weight - user.weight;
        }

        require(storage.threshold <= storage.total_weight, InitError::TotalWeightCannotBeLessThanThreshold);

        // DRY, if they set the same weight then they pay the extra `write` operation
        storage.weighting.insert(user.address, user.weight);
        storage.nonce += 1;

        log(SetWeightEvent { user })
    }

    #[storage(read, write)]
    fn transfer(
        asset_id: ContractId,
        data: b256,
        signatures: Vec<SignatureInfo>,
        to: Identity,
        value: u64,
    ) {
        require(storage.nonce != 0, InitError::NotInitialized);
        require(value <= this_balance(asset_id), ExecutionError::InsufficientAssetAmount);

        let transaction_hash = hash_transaction(data, storage.nonce, to, value);
        let approval_count = count_approvals(signatures, transaction_hash);
        require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

        storage.nonce += 1;

        transfer(value, asset_id, to);

        log(TransferEvent {
            asset: asset_id,
            nonce: storage.nonce - 1,
            to,
            value,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn approval_weight(user: b256) -> u64 {
        storage.weighting.get(user).unwrap_or(0)
    }

    fn balance(asset_id: ContractId) -> u64 {
        this_balance(asset_id)
    }

    #[storage(read)]
    fn nonce() -> u64 {
        storage.nonce
    }

    #[storage(read)]
    fn threshold() -> u64 {
        storage.threshold
    }

    fn transaction_hash(data: b256, nonce: u64, to: Identity, value: u64) -> b256 {
        hash_transaction(data, nonce, to, value)
    }

    fn threshold_hash(data: Option<b256>, nonce: u64, threshold: u64) -> b256 {
        hash_threshold(data, nonce, threshold)
    }

    fn weight_hash(data: Option<b256>, nonce: u64, user: User) -> b256 {
        hash_weight(data, nonce, user)
    }
}

/// Takes in a transaction hash and signatures with associated data.
/// Recovers a b256 address from each signature;
/// it then increments the number of approvals by that address' approval weighting.
/// Returns the final approval count.
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
        approval_count += storage.weighting.get(signer).unwrap_or(0);

        if storage.threshold <= approval_count {
            break;
        }

        index += 1;
    }
    approval_count
}
