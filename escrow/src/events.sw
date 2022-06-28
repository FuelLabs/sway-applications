library events;

dep data_structures;

use std::{contract_id::ContractId, identity::Identity};
use data_structures::EscrowInfo;

pub struct ApproveEvent {
    /// Unique escrow identifier
    identifier: u64,

    /// The user that has approved
    user: Identity,
}

pub struct CreatedEscrowEvent {
    /// The user that has created the escrow
    author: Identity,

    // Metadata for the newly created escrow
    escrow: EscrowInfo,

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

pub struct NewUserEscrowEvent {
    /// Unique escrow identifier
    identifier: u64,

    /// The user that can interact with the escrow
    user: Identity
}

pub struct PaymentTakenEvent {
    /// The amount that has been transferred to the seller
    amount: u64, 

    /// The asset that has been transferred to the seller
    asset: ContractId,

    /// The user who is labelled as the buyer in this exchange
    buyer: Identity, 

    /// Unique escrow identifier
    identifier: u64,

    /// The user who is labelled as the seller in this exchange 
    seller: Identity,
}

pub struct TransferredToSellerEvent {
    /// The amount that has been transferred to the seller
    amount: u64, 

    /// The asset that has been transferred to the seller
    asset: ContractId,

    /// The user who is labelled as the buyer in this exchange
    buyer: Identity, 

    /// Unique escrow identifier
    identifier: u64,

    /// The user who is labelled as the seller in this exchange 
    seller: Identity,
}

pub struct ThresholdReachedEvent {
    /// Unique escrow identifier
    identifier: u64,
}

pub struct WithdrawEvent {
    /// The amount withdrawn by the user
    amount: u64,

    /// The asset that the user has withdrawn
    asset: ContractId,

    /// Unique escrow identifier
    identifier: u64,

    /// The user that has withdrawn
    user: Identity,
}
