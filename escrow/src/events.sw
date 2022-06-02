library events;

use std::{chain::auth::Sender, contract_id::ContractId};

pub struct ApproveEvent {
    user: Sender,
    count: u64,
}

pub struct DepositEvent {
    user: Sender,
    asset: ContractId,
    amount: u64,
}

pub struct ThresholdReachedEvent {
}

pub struct WithdrawEvent {
    user: Sender,
    asset: ContractId,
    amount: u64,
    approval_count: u64,
}
