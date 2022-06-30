library data_structures;

use std::{contract_id::ContractId, identity::Identity};
use core::ops::Eq;

struct CallData {
    /// Data to pass into the called function
    arguments: u64,
    /// Encoded representation of a function to be called on the specified contract
    function_selector: u64,
    /// Id of contract which will be called if a proposal is approved
    /// The contract will be caled using the provided function selector and arguments
    id: ContractId,
}

pub struct Proposal {
    /// Number of coins to forward
    /// Coin type is specified by the `asset` below
    amount: u64,
    /// Asset Id of the coins to forward
    asset: ContractId,
    /// Stores information about an arbitrary contract function call
    call_data: CallData,
    /// Specifies the amount of gas to forward to the arbitrary function call
    gas: u64,
}

pub struct ProposalInfo {
    /// The needed percentage of yes votes to execute a proposal.
    /// 0 < acceptance_percentage <= 100
    acceptance_percentage: u64,
    /// Address or contract which created the proposal
    author: Identity,
    /// Amount of blocks a proposal is valid for after creation
    deadline: u64,
    /// The number of no votes for a proposal
    no_votes: u64,
    /// Data necessary to execute an arbitrary transaction.
    proposal_transaction: Proposal,
    /// The number of yes votes for a proposal
    yes_votes: u64,
}

pub enum State {
    NotInitialized: (),
    Initialized: (),
}

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match(self, other) {
            (State::Initialized, State::Initialized) => true, (State::NotInitialized, State::NotInitialized) => true, _ => false, 
        }
    }
}
