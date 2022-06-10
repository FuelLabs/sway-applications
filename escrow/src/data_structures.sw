library data_structures;

use std::{contract_id::ContractId, identity::Identity, option::*};
use core::ops::Eq;

pub enum State {
    /// Escrow is awaiting calls to deposit() & approve() from all parties
    Pending: (),

    /// All parties have deposited and approved
    Completed: (),
}

pub struct Asset {
    /// Amount of asset the user must deposit
    amount: u64,

    /// The id used to identify the asset for deposit
    id: ContractId,
}

pub struct EscrowData {
    /// Current number of successful calls to approve()
    approval_count: u64,

    /// The assets that this escrow accepts with their required quantities
    assets: [Asset; 2],

    /// Mechanism used to manage the control flow of the escrow
    state: State,

    /// Required number of successful calls to approve() to mark the workflow as complete
    threshold: u64,

    /// The authorized users who are able to interact with this escrow
    users: [Identity; 2],
}

pub struct User {
    /// Flag tracking whether the user has successfully called approve() and is currently in the
    /// approved state (has not reset it via withdrawing)
    approved: bool,

    /// The asset that the user has currently deposited in the contract
    asset: Option<ContractId>,

    /// Dummy value used to ensure that a caller is a valid user
    exists: bool,

    /// Value indicating whether the user currently holds a deposit in the contract
    deposited: bool,
}

pub struct UserEscrows {
    // TODO: must consider "active" being too large to traverse everntually because
    //       we will be moving active -> completed

    /// Array containing unique escrow identifiers for escrows that are State::Pending
    active: [u64; 1],

    /// Array containing unique escrow identifiers for escrows that are State::Completed
    completed: [u64; 1],
}

// TODO: match not implemented on self yet so it won't compile
// impl Eq for State {
//     fn eq(self, other: Self) -> bool {
//         match (self, other) {
//             (State::Void, State::Void) => true,
//             (State::Pending, State::Pending) => true,
//             (State::Completed, State::Completed) => true,
//             _ => false,
//         }
//     }
// }
