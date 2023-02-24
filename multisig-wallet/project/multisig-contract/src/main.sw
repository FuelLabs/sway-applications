contract;

dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use std::{
    auth::msg_sender,
    bytes::Bytes,
    call_frames::contract_id,
    constants::{
        BASE_ASSET_ID,
        ZERO_B256,
    },
    context::this_balance,
    error_signals::FAILED_REQUIRE_SIGNAL,
    hash::sha256,
    logging::log,
    low_level_call::{
        call_with_function_selector,
        CallParams,
    },
    token::transfer,
};

impl Bytes {
    ////////////////////////////////////// not in this forc version /////////////////////////////////////////////////////
    pub fn sha256(self) -> b256 {
        let mut result_buffer = b256::min();
        asm(hash: result_buffer, ptr: self.buf.ptr, bytes: self.len) {
            s256 hash ptr bytes;
            hash: b256
        }
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
}

use data_structures::{SignatureInfo, Transaction, TypeToHash, User};
use errors::{AccessControlError, ExecutionError, InitError};
use events::{CallEvent, CancelEvent, SetThresholdEvent, TransferEvent};
use interface::{Info, MultiSignatureWallet};
use utils::recover_signer;

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
            Identity::ContractId(contract_identifier) => contract_identifier.value,
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
    fn execute_transaction( // TODO: Convert `Option<Vec<u8>>` to `Option<Bytes>` when SDK supports `Bytes`. https://github.com/FuelLabs/fuels-rs/issues/723.
        asset_id: Option<ContractId>,
        calldata: Option<Vec<u8>>,
        forwarded_gas: Option<u64>,
        function_selector: Option<Vec<u8>>,
        signatures: Vec<SignatureInfo>,
        single_value_type_arg: Option<bool>,
        target: Identity,
        value: Option<u64>,
    ) {
        require(storage.nonce != 0, InitError::NotInitialized);

        if function_selector.is_none() {
            require(asset_id.is_some(), ExecutionError::TransferRequiresAnAssetId);
            require(value.is_some(), ExecutionError::TransferRequiresAValue);
            let asset_id = asset_id.unwrap();
            let value = value.unwrap();

            require(value <= this_balance(asset_id), ExecutionError::InsufficientAssetAmount);

            let transaction_hash = Transaction {
                contract_identifier: contract_id(),
                nonce: storage.nonce,
                value: Option::Some(value),
                asset_id: Option::Some(asset_id),
                target,
                function_selector: Option::None,
                calldata: match calldata {
                    Option::None => Option::None,
                    Option::Some(vec) => {
                        let mut vec = vec;
                        Option::Some(Bytes::from_vec_u8(vec))
                    },
                },
                single_value_type_arg,
                forwarded_gas,
            }.into_bytes().sha256();
            let approval_count = count_approvals(signatures, transaction_hash);
            require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

            storage.nonce += 1;

            transfer(value, asset_id, target);

            log(TransferEvent {
                asset: asset_id,
                nonce: storage.nonce - 1,
                target,
                value,
            });
        } else if function_selector.is_some() {
            let target_contract_id = match target {
                Identity::ContractId(contract_identifier) => contract_identifier,
                Identity::Address => {
                    log(ExecutionError::CannotCallFunctionsOnAddresses);
                    revert(FAILED_REQUIRE_SIGNAL)
                },
            };

            require(calldata.is_some(), ExecutionError::CallingFunctionsRequiresCalldata);
            require(single_value_type_arg.is_some(), ExecutionError::CallingFunctionsRequiresSingleValueTypeArg);
            let function_selector = Bytes::from_vec_u8(function_selector.unwrap());
            let calldata = Bytes::from_vec_u8(calldata.unwrap());
            let single_value_type_arg = single_value_type_arg.unwrap();

            if value.is_some() {
                require(asset_id.is_some(), ExecutionError::TransferRequiresAnAssetId);
                require(value.unwrap() <= this_balance(asset_id.unwrap()), ExecutionError::InsufficientAssetAmount);
            }

            let transaction_hash = Transaction {
                contract_identifier: contract_id(),
                nonce: storage.nonce,
                value,
                asset_id,
                target,
                function_selector: Option::Some(function_selector),
                calldata: Option::Some(calldata),
                single_value_type_arg: Option::Some(single_value_type_arg),
                forwarded_gas,
            }.into_bytes().sha256();
            let approval_count = count_approvals(signatures, transaction_hash);
            require(storage.threshold <= approval_count, ExecutionError::InsufficientApprovals);

            let transaction_nonce = storage.nonce;
            storage.nonce += 1;

            let call_params = CallParams {
                coins: value.unwrap_or(0),
                asset_id: asset_id.unwrap_or(BASE_ASSET_ID),
                gas: forwarded_gas.unwrap_or(0),
            };
            call_with_function_selector(target_contract_id, function_selector, calldata, single_value_type_arg, call_params);

            log(CallEvent {
                call_params,
                nonce: transaction_nonce,
                target_contract_id,
                // function_selector: function_selector.into_vec_u8(),
                // calldata: calldata.into_vec_u8(),
            });
        }
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

    fn compute_hash(type_to_hash: TypeToHash) -> b256 { // Currently won't work for the `Transaction` type as the SDK doesn't support `Bytes` (https://github.com/FuelLabs/fuels-rs/issues/723), 
                                                        // to hash `Transaction` use `compute_transaction_hash` instead.
        match type_to_hash {
            //TypeToHash::Transaction(transaction) => transaction.into_bytes().sha256(),
            TypeToHash::User(user) => sha256(user),
        }
    }

    fn compute_transaction_hash( // Needed for hashing the `Transaction` type, as `Bytes are not supported by SDK`, and Vectors as fields in a struct are not supported by the SDK.
                                 // Once `Bytes` are supported in the SDK, this can be deprecated and compute_hash can be used for hashing the `Transaction` type.
        contract_identifier: ContractId,
        nonce: u64,
        value: Option<u64>,
        asset_id: Option<ContractId>,
        target: Identity,
        function_selector: Option<Vec<u8>>,
        calldata: Option<Vec<u8>>,
        single_value_type_arg: Option<bool>,
        forwarded_gas: Option<u64>,
    ) -> b256 {
        Transaction {
            contract_identifier,
            nonce,
            value,
            asset_id,
            target,
            function_selector: match function_selector {
                Option::None => Option::None,
                Option::Some(vec) => {
                    let mut vec = vec;
                    Option::Some(Bytes::from_vec_u8(vec))
                },
            },
            calldata: match calldata {
                Option::None => Option::None,
                Option::Some(vec) => {
                    let mut vec = vec;
                    Option::Some(Bytes::from_vec_u8(vec))
                },
            },
            single_value_type_arg,
            forwarded_gas,
        }.into_bytes().sha256()
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
