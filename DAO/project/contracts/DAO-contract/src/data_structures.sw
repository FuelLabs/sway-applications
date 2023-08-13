library;

use std::block::height;
use core::ops::Eq;

/// CallData stores information about an arbitrary contract call
struct CallData {
    /// Data to pass into the called function
    arguments: u64,
    /// Encoded representation of a function to be called on the specified contract
    function_selector: u64,
    /// Id of contract which will be called if a proposal is approved
    /// The contract will be called using the provided function selector and arguments
    id: ContractId,
}

/// A Proposed transaction to be executed if the proposal is accepted
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

/// The voting information for a proposal.
pub struct ProposalInfo {
    /// The needed percentage of yes votes to execute a proposal.
    /// 0 < acceptance_percentage <= 100
    acceptance_percentage: u64,
    /// Address or contract which created the proposal
    author: Identity,
    /// Represents an end time (block height) for proposals
    /// Proposals can be voted on as long as the block height has not exceeded the deadline
    deadline: u64,
    /// Whether the proposal has been executed
    executed: bool,
    /// The number of no votes for a proposal
    no_votes: u64,
    /// Data necessary to execute an arbitrary transaction.
    proposal_transaction: Proposal,
    /// The number of yes votes for a proposal
    yes_votes: u64,
}

impl ProposalInfo {
    /// Creates a new ProposalInfo.
    ///
    /// # Arguments
    ///
    /// * `acceptance_percentage`: [u64] - The needed percentage of yes votes to execute a proposal.
    /// * `author`: [Identity] - Identity which created the proposal.
    /// * `duration`: [u64] - The duration of the proposal in blocks.
    /// * `proposal_transaction`: [Proposal] - The transaction to be executed if the proposal is accepted.
    ///
    /// # Returns
    ///
    /// * [ProposalInfo] - A new ProposalInfo.
    pub fn new(
        acceptance_percentage: u64,
        author: Identity,
        duration: u64,
        proposal_transaction: Proposal,
    ) -> Self {
        ProposalInfo {
            acceptance_percentage,
            author,
            deadline: duration + height(),
            executed: false,
            no_votes: 0,
            proposal_transaction,
            yes_votes: 0,
        }
    }
}

/// Represents whether the contract has been initialized.
pub enum State {
    /// The contract has not been initialized.
    NotInitialized: (),
    /// The contract has been initialized.
    Initialized: (),
}

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (State::Initialized, State::Initialized) => true,
            (State::NotInitialized, State::NotInitialized) => true,
            _ => false,
        }
    }
}

/// Stores the number and type of votes for a proposal.
pub struct Votes {
    /// Stores the number of no votes for a proposal.
    no_votes: u64,
    /// Stores the number of yes votes for a proposal.
    yes_votes: u64,
}

impl Votes {
    /// Returns a new Votes struct with 0 yes/no votes.
    ///
    /// # Returns
    ///
    /// * [Votes] - A new Votes struct with no votes.
    pub fn default() -> Self {
        Self {
            no_votes: 0,
            yes_votes: 0,
        }
    }
}
