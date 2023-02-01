library events;

use std::{bytes::Bytes, low_level_call::CallParams};

pub struct CancelEvent {
    cancelled_nonce: u64,
    user: b256,
}

pub struct CallEvent {
    call_params: CallParams,
    nonce: u64,
    target_contract_id: ContractId,
    function_selector: Bytes,
    calldata: Bytes,
}

pub struct SetThresholdEvent {
    previous_threshold: u64,
    threshold: u64,
}

pub struct TransferEvent {
    asset: ContractId,
    nonce: u64,
    target: Identity,
    value: u64,
}
