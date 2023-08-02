library;

use ::data_structures::{hashing::{ContractCallParams, TransferParams}, user::User};
use std::{bytes::Bytes, low_level_call::CallParams};

/// Log of an executed transaction.
pub struct ExecuteTransactionEvent {
    // contract_call_params: contract_call_params, // TODO: Uncomment when SDK supports logs with nested Bytes https://github.com/FuelLabs/fuels-rs/issues/1046
    /// The nonce of the transaction.
    nonce: u64,
    /// The target of the transaction.
    target: Identity,
    /// The parameters for the transfer of value in the transaction.
    transfer_params: TransferParams,
}

/// Log of setting the threshold.
pub struct SetThresholdEvent {
    /// The nonce of the transaction.
    nonce: u64,
    /// The previous threshold.
    previous_threshold: u64,
    /// The new threshold.
    threshold: u64,
}

/// Log of setting the threshold.
pub struct SetWeightEvent {
    /// The nonce of the transaction.
    nonce: u64,
    /// The information of user who's weight has been changed.
    user: User,
}
