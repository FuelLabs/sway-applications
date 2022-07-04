library data_structures;

// TODO: docs

use core::ops::Eq;
use std::{contract_id::ContractId, identity::Identity, option::Option, vec::Vec};

pub struct Arbiter {
    address: Identity,
    fee_percentage: u64,
}

impl Eq for Arbiter {
    fn eq(self, other: Self) -> bool {
        self.address == other.address && self.fee_percentage == other.fee_percentage
    }
}

pub struct ArbiterProposal {
    /// New arbiter proposal made by the buyer
    buyer: Option<Arbiter>,

    /// New arbiter proposal made by the seller
    seller: Option<Arbiter>,
}

pub struct Asset {
    /// Amount of asset the user must deposit
    amount: u64,

    /// The id used to identify the asset for deposit
    id: ContractId,
}

pub struct Buyer {
    address: Identity,

    /// The asset that the user has currently deposited in the contract
    asset: Option<ContractId>,

    // Minor data duplication allows us to not bother validating unique assets upon escrow creation
    // otherwise the same asset with different values can be added which, if handled incorrectly,
    // may allow the user to drain the contract
    /// The amount of asset that has been deposited
    deposited_amount: u64,

    // when additional fee is added SELLER will also need this
    disputed: bool,
}

pub struct EscrowInfo {
    /// Trusted 3rd party who handles the resolution of a dispute
    arbiter: Identity,

    /// The fee (as a percentage) paid to the arbiter upon handling a dispute
    arbiter_fee_percentage: u64,

    /// The assets that the escrow accepts with their required quantities
    /// This allows the buyers to select which asset they want to deposit
    assets: Vec<Asset>,

    /// The authorized user who is able to make a payment into the escrow
    buyer: Buyer,

    ///
    deadline: u64,

    ///
    disputed: bool,

    /// The authorized user who is the recipient of payments made by the buyer
    seller: Seller,

    /// Mechanism used to manage the control flow of the escrow
    state: State,
}

pub struct Seller {
    address: Identity,

    disputed: bool,
}

pub enum State {
    /// Escrow is awaiting calls to deposit() & approve() from all parties
    Pending: (),

    /// All parties have deposited and approved
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
