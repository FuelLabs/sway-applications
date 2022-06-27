library events;

dep data_structures;

use std::identity::Identity;
use data_structures::ProposalInfo;

pub struct CreatedProposalEvent {
    /// The user who created the proposal
    author: Identity,

    /// The unique Identifier for the proposal
    id: u64,

    /// Information about the proposal
    proposal_info: ProposalInfo,
}

pub struct DepositEvent {
    /// The amount deposited
    amount: u64,

    /// The user who deposited
    user: Identity,
}

pub struct ExecuteEvent {
    /// The unique identifier for the proposal
    id: u64,
}

pub struct UnlockVotesEvent {
    /// The unique identifier for the proposal
    id: u64,

    /// Amount of votes unlocked
    vote_amount: u64,
}

pub struct VoteEvent {
    /// The unique identifier for the proposal
    id: u64,

    /// The use who voted
    user: Identity,

    /// The amount of votes to add to the proposal
    vote_amount: u64,
}

pub struct WithdrawEvent {
    /// The amount withdrawn
    amount: u64,

    /// The user who withdrew
    user: Identity,
}
