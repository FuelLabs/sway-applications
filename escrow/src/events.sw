library events;

use std::{contract_id::ContractId, identity::Identity};

pub struct ApproveEvent {
    count: u64,
    user: Identity,
}

pub struct DepositEvent {
    amount: u64,
    asset: ContractId,
    user: Identity,
}

pub struct ThresholdReachedEvent {
}

pub struct WithdrawEvent {
    amount: u64,
    approval_count: u64,
    asset: ContractId,
    user: Identity,
}
