library data_structures;

use std::{contract_id::ContractId, option::*};
use core::ops::Eq;

/// Control flow mechanism for the contract
pub enum State {
    Completed: (),
    Pending: (),
    Void: (),
}

/// Native asset the user must deposit and amount required for despoit
pub struct Asset {
    amount: u64,
    id: ContractId,
}

/// Metadata stored in mapping for identification and logic
pub struct User {
    approved: bool,

    /// The asset that the user has currently deposited in the contract
    asset: Option<ContractId>,

    /// Dummy value used to ensure that a caller is a valid user
    exists: bool,

    /// Value indicating whether the user currently holds a deposit in the contract
    deposited: bool,
}

impl Eq for User {
    fn eq(self, other: Self) -> bool {
        let p1 = self.approved == other.approved && self.exists == other.exists && self.deposited == other.deposited;

        if !p1 {
            return false;
        }

        // workaround
        match self.asset {
            Option::Some(asset1) => {
                match other.asset {
                    Option::Some(asset2) => asset1 == asset2,
                    _ => false,
                }
            },
            Option::None(_) => {
                match other.asset {
                    Option::None(_) => true,
                    _ => false,
                }
            }
        }
    }
}

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (State::Void, State::Void) => true,
            (State::Pending, State::Pending) => true,
            (State::Completed, State::Completed) => true,
            _ => false,
        }
    }
}
