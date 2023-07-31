library;

use ::data_structures::{hashing::{ContractCallParams, TransferParams}, user::User};
use std::{bytes::Bytes, low_level_call::CallParams};

pub struct ExecuteTransactionEvent {
    contract_call_params: Option<ContractCallParams>,
    nonce: u64,
    target: Identity,
    transfer_params: TransferParams,
}

pub struct SetThresholdEvent {
    nonce: u64,
    previous_threshold: u64,
    threshold: u64,
}

pub struct SetWeightEvent {
    nonce: u64,
    user: User,
}
