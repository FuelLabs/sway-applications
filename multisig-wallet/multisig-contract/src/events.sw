library;

use ::data_structures::{hashing::TransactionParameters, user::User};
use std::{bytes::Bytes, low_level_call::CallParams};

/// Log of an executed transaction.
pub struct ExecuteTransactionEvent {
    /// The nonce of the transaction.
    nonce: u64,
    /// The parameters of the transaction.
    /// The target of the transaction.
    // transaction_parameters: TransactionParameters, // TODO: Uncomment and reorder fields when SDK supports logs with nested Bytes https://github.com/FuelLabs/fuels-rs/issues/1046
    target: Identity,
}

/// Log of setting the threshold.
pub struct SetThresholdEvent {
    /// The previous threshold.
    previous_threshold: u64,
    /// The new threshold.
    threshold: u64,
}

/// Log of setting the threshold.
pub struct SetWeightEvent {
    /// The information of user who's weight has been changed.
    user: User,
}
