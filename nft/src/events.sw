library events;

use std::address::Address;

pub struct ApprovalEvent {
    owner: Address,
    approved: Address,
    token_id: u64
}

pub struct BurnEvent {
    owner: Address,
    token_id: u64
}

pub struct MintEvent {
    owner: Address,
    token_id: u64
}

pub struct OperatorEvent {
    owner: Address,
    operator: Address
}

pub struct TransferEvent {
    from: Address,
    to: Address,
    token_id: u64
}
