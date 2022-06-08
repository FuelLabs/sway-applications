library events;

use std::identity::Identity;

pub struct ApprovalEvent {
    owner: Identity,
    approved: Identity,
    token_id: u64
}

pub struct BurnEvent {
    owner: Identity,
    token_id: u64
}

pub struct MintEvent {
    owner: Identity,
    token_id: u64
}

pub struct OperatorEvent {
    owner: Identity,
    operator: Identity
}

pub struct TransferEvent {
    from: Identity,
    to: Identity,
    token_id: u64
}
