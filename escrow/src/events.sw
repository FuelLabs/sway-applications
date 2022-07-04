library events;

dep data_structures;

use data_structures::{Arbiter, EscrowInfo};
use std::{contract_id::ContractId, identity::Identity};

pub struct ChangedArbiterEvent {
    /// The address of the new arbiter
    address: Identity,

    /// The fee (%) that the new arbiter takes upon resolving a dispute
    fee_percentage: u64,

    /// Unique escrow identifier
    identifier: u64,
}

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

pub struct ProposedArbiterEvent {
    /// Arbiter address and fee %
    arbiter: Arbiter,

    /// Unique escrow identifier
    identifier: u64,

    /// Buyer or seller
    user: Identity,
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
