contract;

// TODO:
//      - change the "data" in the Tx hashing from b256 to Bytes type when SDK support is implemented: https://github.com/FuelLabs/fuels-rs/issues/723.
//    
dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use std::{
    auth::msg_sender,
    call_frames::contract_id,
    constants::ZERO_B256,
    context::this_balance,
    logging::log,
    low_level_call::{call_with_function_selector_vec, CallParams},
    token::transfer,
};

use data_structures::{SignatureInfo, User};
use errors::{AccessControlError, ExecutionError, InitError};
use events::{CancelEvent, ExecutedEvent, SetThresholdEvent, TransferEvent};
use interface::{Info, MultiSignatureWallet};
use utils::{create_hash, create_payload, recover_signer};

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
    fn cancel_transaction() {
        let sender = match msg_sender().unwrap() {
            Identity::Address(address) => address.value,
            Identity::ContractId(asset_id) => asset_id.value,
        };
        require(storage.weighting.get(sender) > 0, AccessControlError::CanOnlyBeAccessedByAnOwner);

        storage.nonce += 1;

        log(CancelEvent {
            cancelled_nonce: storage.nonce - 1,
            user: sender,
        });
    }

    #[storage(read, write)]
    fn constructor(users: Vec<User>) {
        require(storage.nonce == 0, InitError::CannotReinitialize);
        require(THRESHOLD != 0, InitError::ThresholdCannotBeZero);

        let mut user_index = 0;
        let mut total_weight = 0;
        while user_index < users.len() {
            require(ZERO_B256 != users.get(user_index).unwrap().address, InitError::AddressCannotBeZero);
            require(users.get(user_index).unwrap().weight != 0, InitError::WeightingCannotBeZero);

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
        asset_id: Option<ContractId>,
        calldata: Option<Vec<u8>>,
        function_selector: Option<Vec<u8>>,
        forwarded_gas: Option<u64>,
        signatures: Vec<SignatureInfo>, 
        single_value_type_arg: Option<bool>,
        target: Identity, 
        value: Option<u64>,
    ) {
        require(storage.nonce != 0, InitError::NotInitialized);

        if function_selector.is_none() {

            //transfer
            require(asset_id.is_some(), ExecutionError::TransferRequiresAnAssetId); 
            require(value.is_some(), ExecutionError::TransferRequiresAValue); 
            
            let asset_id = asset_id.unwrap();
            let value = value.unwrap();

            require(value <= this_balance(asset_id), ExecutionError::InsufficientAssetAmount);

            let data = match target {
                Identity::Address(address) => address_to_bytes(address),
                Identity::ContractId(contract_id) => contract_id_to_bytes(contract_id),
            };

            let transaction_hash = create_hash(data, storage.nonce, target, value);
            let approval_count = count_approvals(signatures, transaction_hash);
            require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

            storage.nonce += 1;

            transfer(value, asset_id, target);

            log(TransferEvent {
                asset: asset_id,
                nonce: storage.nonce - 1,
                to: target,
                value,
            });

        } else {

            //call


        }

    }

    #[storage(read, write)]
    fn set_threshold(
        data: b256,
        nonce: u64,
        signatures: Vec<SignatureInfo>,
        threshold: u64,
    ) {
        require(storage.nonce != 0, InitError::NotInitialized);
        require(threshold != 0, InitError::ThresholdCannotBeZero);
        require(threshold <= storage.total_weight, InitError::TotalWeightCannotBeLessThanThreshold);

        let transaction_hash = create_hash(data, nonce, Identity::ContractId(contract_id()), 0);
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
    fn transfer(
        asset_id: ContractId,
        data: b256,
        signatures: Vec<SignatureInfo>,
        to: Identity,
        value: u64,
    ) {
        require(storage.nonce != 0, InitError::NotInitialized);
        require(value <= this_balance(asset_id), ExecutionError::InsufficientAssetAmount);

        let transaction_hash = create_hash(data, storage.nonce, to, value);
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

    fn transaction_hash(data: Vec<u8>, nonce: u64, to: Identity, value: u64) -> b256 { 
        create_hash(Bytes::from_vec_u8(data), nonce, to, value) // TO DO : Switch Vec<u8> to Bytes when SDK supports Bytes
    }

    fn update_hash(data: b256, nonce: u64) -> b256 {
        // Assume default values for `to` and `value` to simplify the abi user experience
        create_hash(data, nonce, Identity::ContractId(contract_id()), 0)
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
        approval_count += storage.weighting.get(signer);

        if storage.threshold <= approval_count {
            break;
        }

        index += 1;
    }
    approval_count
}
