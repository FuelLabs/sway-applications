library data_structures;

use std::{contract_id::ContractId, identity::Identity, option::Option, vec::Vec};
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

pub struct EscrowInfo {
    /// Current number of successful calls to approve()
    approval_count: u64,

    /// The assets that this escrow accepts with their required quantities
    assets: Vec<Asset>,

    /// Mechanism used to manage the control flow of the escrow
    state: State,

    /// Required number of successful calls to approve() to mark the workflow as complete
    threshold: u64,

    /// The authorized users who are able to interact with this escrow
    users: Vec<Identity>,
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

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match(self, other) {
            (State::Pending, State::Pending) => true, (State::Completed, State::Completed) => true, _ => false, 
        }
    }
}
