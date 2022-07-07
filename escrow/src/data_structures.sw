library data_structures;

use core::ops::Eq;
use std::{contract_id::ContractId, identity::Identity, option::Option, vec::Vec};

pub struct Arbiter {
    /// Address identifying the arbiter
    address: Identity,

    /// The asset that the arbiter will be paid in upon resolution
    asset: ContractId,

    /// The quantity of asset to be taken as payment
    fee_amount: u64,
}

impl Eq for Arbiter {
    fn eq(self, other: Self) -> bool {
        self.address == other.address && self.asset == other.asset && self.fee_amount == other.fee_amount
    }
}

pub struct Asset {
    /// Amount of asset the user must deposit
    amount: u64,

    /// The id used to identify the asset for deposit
    id: ContractId,
}

pub struct Buyer {
    /// Address identifying the buyer
    address: Identity,

    /// The asset that the user has currently deposited in the contract
    asset: Option<ContractId>,

    // Minor data duplication allows us to forego validating unique assets upon escrow creation
    // otherwise the same asset with different values can be added which, if handled incorrectly,
    // may allow the user to drain the contract
    /// The amount of asset that has been deposited
    deposited_amount: u64,
}

pub struct EscrowInfo {
    /// Trusted 3rd party who handles the resolution of a dispute
    arbiter: Arbiter,

    /// The assets that the escrow accepts with their required quantities
    /// This allows the buyer to select which asset they want to deposit
    assets: Vec<Asset>,

    /// The authorized user who is able to make a payment into the escrow
    buyer: Buyer,

    /// End height after which the buyer can no longer deposit and the seller can take payment
    deadline: u64,

    /// Marker set by the buyer to lock the escrow and prevent the seller from taking payment
    disputed: bool,

    /// The authorized user who is the recipient of payments made by the buyer
    seller: Seller,

    /// Mechanism used to manage the control flow of the escrow
    state: State,
}

pub struct Seller {
    /// Address identifying the seller
    address: Identity,
}

pub enum State {
    /// Escrow has been created however the deposit has not been sent to either buyer or seller
    Pending: (),

    /// The deposit has been sent to either the buyer or seller
    Completed: (),
}

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match(self, other) {
            (State::Pending, State::Pending) => {
                true
            },
            (State::Completed, State::Completed) => {
                true
            },
            _ => {
                false
            },
        }
    }
}
