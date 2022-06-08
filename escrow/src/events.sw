library events;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};
use data_structures::EscrowData;

pub struct ApproveEvent {
    /// The number of approvals at the time of event
    approval_count: u64,

    /// Unique escrow identifier
    identifier: u64,

    /// The user that has approved
    user: Identity,
}

pub struct CreatedEscrowEvent {
    // Metadata for the newly created escrow
    escrow: EscrowData,

    /// Unique escrow identifier
    identifier: u64,
}

pub struct DepositEvent {
    /// The amount deposited by the user
    amount: u64,

    /// The asset that the user deposited
    asset: ContractId,

    /// Unique escrow identifier
    identifier: u64,

    /// The user that has deposited
    user: Identity,
}

pub struct ThresholdReachedEvent {
    /// Unique escrow identifier
    identifier: u64,
}

pub struct WithdrawEvent {
    /// The amount withdrawn by the user
    amount: u64,

    /// The number of approvals at the time of event
    approval_count: u64,

    /// The asset that the user has withdrawn
    asset: ContractId,

    /// Unique escrow identifier
    identifier: u64,

    /// The user that has withdrawn
    user: Identity,
}
