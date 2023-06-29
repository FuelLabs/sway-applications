library;

use ::data_structures::ProposalInfo;

pub struct CreateProposalEvent {
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
    /// Actual acceptance percentage of approved proposal
    acceptance_percentage: u64,
    /// The unique identifier for the proposal
    id: u64,
    /// User who executed the event
    user: Identity,
}

pub struct InitializeEvent {
    /// User who initialized the contract
    author: Identity,
    /// Contract Id of the token used for DAO governance
    token: ContractId,
}

pub struct UnlockVotesEvent {
    /// The unique identifier for the proposal
    id: u64,
    /// User who unlocks the tokens
    user: Identity,
    /// Amount of votes unlocked
    vote_amount: u64,
}

pub struct VoteEvent {
    /// The unique identifier for the proposal
    id: u64,
    /// The user who voted
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
