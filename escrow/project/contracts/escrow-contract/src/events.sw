library events;

dep data_structures;

use data_structures::{Arbiter, EscrowInfo};

pub struct AcceptedArbiterEvent {
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
    /// Data describing the address, asset to be paid in and amount of asset (fee)
    arbiter: Arbiter,
    /// Unique escrow identifier
    identifier: u64,
}

pub struct ResolvedDisputeEvent {
    /// Unique escrow identifier
    identifier: u64,
    /// The user that has been chosen by the arbiter to receive the disputed funds (buyer / seller)
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

pub struct WithdrawnCollateralEvent {
    /// Unique escrow identifier
    identifier: u64,
}
