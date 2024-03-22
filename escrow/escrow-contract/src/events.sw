library;

use ::data_structures::{Arbiter, EscrowInfo};

/// Event for when an arbiter is accepted.
pub struct AcceptedArbiterEvent {
    /// Unique escrow identifier.
    identifier: u64,
}

/// Event for when an escrow is created.
pub struct CreatedEscrowEvent {
    // Metadata for the newly created escrow.
    escrow: EscrowInfo,
    /// Unique escrow identifier.
    identifier: u64,
}

/// Event for when a deposit is made.
pub struct DepositEvent {
    /// The asset that the user deposited.
    asset: AssetId,
    /// Unique escrow identifier.
    identifier: u64,
}

/// Event for when a dispute is raised.
pub struct DisputeEvent {
    /// Unique escrow identifier.
    identifier: u64,
}

/// Event for a payment is taken.
pub struct PaymentTakenEvent {
    /// Unique escrow identifier.
    identifier: u64,
}

/// Event for when an arbiter is proposed.
pub struct ProposedArbiterEvent {
    /// Data describing the address, asset to be paid in and amount of asset (fee).
    arbiter: Arbiter,
    /// Unique escrow identifier.
    identifier: u64,
}

/// Event for when a dispute is resolved.
pub struct ResolvedDisputeEvent {
    /// Unique escrow identifier.
    identifier: u64,
    /// The user that has been chosen by the arbiter to receive the disputed funds (buyer / seller).
    user: Identity,
}

/// Event for when a deposit is returned.
pub struct ReturnedDepositEvent {
    /// Unique escrow identifier.
    identifier: u64,
}

/// Event for when the sellers assets are transferred back.
pub struct TransferredToSellerEvent {
    /// Unique escrow identifier.
    identifier: u64,
}

/// Event for when the collateral is withdrawn.
pub struct WithdrawnCollateralEvent {
    /// Unique escrow identifier.
    identifier: u64,
}
