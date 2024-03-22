contract;

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
        Threshold,
        Transaction,
        TransactionParameters,
        TypeToHash,
        Weight,
    },
    signatures::SignatureInfo,
    user::User,
};
use std::{
    asset::transfer,
    call_frames::contract_id,
    context::this_balance,
    error_signals::FAILED_REQUIRE_SIGNAL,
    hash::{
        Hash,
        sha256,
    },
    low_level_call::{
        call_with_function_selector,
        CallParams,
    },
};
use ::utils::{compute_hash, recover_signer};

configurable {
    /// The threshold required for activation.
    THRESHOLD: u64 = 5,
}

storage {
    /// The nonce of the multisig wallet, used to prevent double spending.
    nonce: u64 = 0,
    /// The total weight of all the user approvals.
    total_weight: u64 = 0,
    /// The number of approvals required in order to execute a transaction.
    ///
    /// # Additional Information
    ///
    /// Set to the value of the configurable `THRESHOLD`.
    threshold: u64 = 0,
    /// Number of approvals per user.
    ///
    /// # Additional Information
    ///
    /// Maps (user => weight).
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
            storage
                .weighting
                .insert(
                    users
                        .get(user_index)
                        .unwrap()
                        .address,
                    users
                        .get(user_index)
                        .unwrap()
                        .weight,
                );
            total_weight += users.get(user_index).unwrap().weight;

            user_index += 1;
        }

        require(
            THRESHOLD <= total_weight,
            InitError::TotalWeightCannotBeLessThanThreshold,
        );

        storage.nonce.write(1);
        storage.threshold.write(THRESHOLD);
        storage.total_weight.write(total_weight);
    }

    #[storage(read, write)]
    fn execute_transaction(
        signatures: Vec<SignatureInfo>,
        target: Identity,
        transaction_parameters: TransactionParameters,
    ) {
        let nonce = storage.nonce.read();
        require(nonce != 0, InitError::NotInitialized);

        match transaction_parameters {
            TransactionParameters::Call(contract_call_params) => {
                let target_contract_id = match target {
                    Identity::ContractId(contract_identifier) => contract_identifier,
                    _ => {
                        log(ExecutionError::CanOnlyCallContracts);
                        revert(FAILED_REQUIRE_SIGNAL)
                    },
                };

                if contract_call_params.transfer_params.value.is_some() {
                    require(
                        contract_call_params
                            .transfer_params
                            .value
                            .unwrap() <= this_balance(contract_call_params.transfer_params.asset_id),
                        ExecutionError::InsufficientAssetAmount,
                    );
                }

                let transaction_hash = compute_hash(TypeToHash::Transaction(Transaction::new(contract_id(), nonce, target, transaction_parameters)));
                let approval_count = count_approvals(signatures, transaction_hash);
                require(
                    storage
                        .threshold
                        .read() <= approval_count,
                    ExecutionError::InsufficientApprovals,
                );

                storage.nonce.write(nonce + 1);

                let call_params = CallParams {
                    coins: contract_call_params.transfer_params.value.unwrap_or(0),
                    asset_id: contract_call_params.transfer_params.asset_id,
                    gas: contract_call_params.forwarded_gas,
                };
                call_with_function_selector(
                    target_contract_id,
                    contract_call_params
                        .function_selector,
                    contract_call_params
                        .calldata,
                    contract_call_params
                        .single_value_type_arg,
                    call_params,
                );
            },
            TransactionParameters::Transfer(transfer_params) => {
                require(
                    transfer_params
                        .value
                        .is_some(),
                    ExecutionError::TransferRequiresAValue,
                );
                let value = transfer_params.value.unwrap();
                require(
                    value <= this_balance(transfer_params.asset_id),
                    ExecutionError::InsufficientAssetAmount,
                );

                let transaction_hash = compute_hash(TypeToHash::Transaction(Transaction::new(contract_id(), nonce, target, transaction_parameters)));
                let approval_count = count_approvals(signatures, transaction_hash);
                require(
                    storage
                        .threshold
                        .read() <= approval_count,
                    ExecutionError::InsufficientApprovals,
                );

                storage.nonce.write(nonce + 1);

                transfer(target, transfer_params.asset_id, value);
            },
        }

        log(ExecuteTransactionEvent { nonce, target
        // transaction_parameters,// TODO: Uncomment when SDK supports logs with nested Bytes https://github.com/FuelLabs/fuels-rs/issues/1046
 });
    }

    #[storage(read, write)]
    fn set_threshold(signatures: Vec<SignatureInfo>, threshold: u64) {
        let nonce = storage.nonce.read();
        require(nonce != 0, InitError::NotInitialized);
        require(threshold != 0, InitError::ThresholdCannotBeZero);
        require(
            threshold <= storage
                .total_weight
                .read(),
            InitError::TotalWeightCannotBeLessThanThreshold,
        );

        let transaction_hash = compute_hash(TypeToHash::Threshold(Threshold::new(contract_id(), nonce, threshold)));
        let approval_count = count_approvals(signatures, transaction_hash);

        let previous_threshold = storage.threshold.read();
        require(
            previous_threshold <= approval_count,
            ExecutionError::InsufficientApprovals,
        );

        storage.nonce.write(nonce + 1);
        storage.threshold.write(threshold);

        log(SetThresholdEvent {
            previous_threshold,
            threshold,
        });
    }

    #[storage(read, write)]
    fn set_weight(signatures: Vec<SignatureInfo>, user: User) {
        let nonce = storage.nonce.read();
        require(nonce != 0, InitError::NotInitialized);

        let transaction_hash = compute_hash(TypeToHash::Weight(Weight::new(contract_id(), nonce, user)));
        let approval_count = count_approvals(signatures, transaction_hash);

        let threshold = storage.threshold.read();
        require(
            threshold <= approval_count,
            ExecutionError::InsufficientApprovals,
        );

        let current_weight = storage.weighting.get(user.address).try_read().unwrap_or(0);

        if current_weight < user.weight {
            storage
                .total_weight
                .write(storage.total_weight.read() + (user.weight - current_weight));
        } else if user.weight < current_weight {
            storage
                .total_weight
                .write(storage.total_weight.read() - (current_weight - user.weight));
        }

        require(
            threshold <= storage
                .total_weight
                .read(),
            InitError::TotalWeightCannotBeLessThanThreshold,
        );

        storage.weighting.insert(user.address, user.weight);
        storage.nonce.write(nonce + 1);

        log(SetWeightEvent { user })
    }
}

impl Info for Contract {
    #[storage(read)]
    fn approval_weight(user: b256) -> u64 {
        storage.weighting.get(user).try_read().unwrap_or(0)
    }

    fn balance(asset_id: AssetId) -> u64 {
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

/// This function counts the number of approvals for a set of users recovered from a set of signatures.
///
/// # Additional Information
///
/// Takes in a transaction hash and signatures with associated data.
/// Recovers a b256 address from each signature;
/// it then increments the number of approvals by that address' approval weighting.
/// Returns the final approval count.
///
/// # Arguments
///
/// * `signatures`: [Vec<SignatureInfo>] - The information for each user's signature for a specific transaction.
/// * `transaction_hash`: [b256] - The hash used to recover a signer.
///
/// # Returns
///
/// * [u64] - The final approval count.
///
/// # Reverts
///
/// * When the public key cannot be recovered from a signature.
/// * When the recovered addresses in `count_approvals `are not in ascending order (0x1 < 0x2 < 0x3...) [b256].
///
/// # Number of Storage Accesses
///
/// * Reads: `2`
#[storage(read)]
fn count_approvals(signatures: Vec<SignatureInfo>, transaction_hash: b256) -> u64 {
    // The signers must have increasing values in order to check for duplicates or a zero-value.
    let mut previous_signer = b256::min();

    let mut approval_count = 0;
    let mut index = 0;
    while index < signatures.len() {
        let signer = recover_signer(transaction_hash, signatures.get(index).unwrap());

        require(
            previous_signer < signer,
            ExecutionError::IncorrectSignerOrdering,
        );

        previous_signer = signer;
        approval_count += storage.weighting.get(signer).try_read().unwrap_or(0);

        if storage.threshold.read() <= approval_count {
            break;
        }

        index += 1;
    }
    approval_count
}
