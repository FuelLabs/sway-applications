library events;

use std::{contract_id::ContractId, identity::Identity};

pub struct ApproveEvent {
    user: Identity,
    count: u64,
}

pub struct DepositEvent {
    user: Identity,
    asset: ContractId,
    amount: u64,
}

pub struct ThresholdReachedEvent {
}

pub struct WithdrawEvent {
    user: Identity,
    asset: ContractId,
    amount: u64,
    approval_count: u64,
}
