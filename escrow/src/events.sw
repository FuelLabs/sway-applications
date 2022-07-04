library events;

dep data_structures;

use data_structures::EscrowInfo;
use std::{contract_id::ContractId, identity::Identity};

pub struct CreatedEscrowEvent {
    // Metadata for the newly created escrow
    escrow: EscrowInfo,

    /// Unique escrow identifier
    identifier: u64,
}

pub struct DepositEvent {
    /// The asset that the user deposited
    asset: ContractId,

    /// Unique escrow identifier
    identifier: u64,
}

pub struct DisputeEvent {
    /// Unique escrow identifier
    identifier: u64,
}

pub struct PaymentTakenEvent {
    /// Unique escrow identifier
    identifier: u64,
}

pub struct ResolvedDisputeEvent {
    /// Unique escrow identifier
    identifier: u64,

    /// The user that can interact with the escrow
    user: Identity,
}

pub struct ReturnedDepositEvent {
    /// Unique escrow identifier
    identifier: u64,
}

pub struct TransferredToSellerEvent {
    /// Unique escrow identifier
    identifier: u64,
}
