library;

use ::data_structures::ProposalInfo;

/// Event for creation of a new proposal.
pub struct CreateProposalEvent {
    /// The unique Identifier for the proposal.
    id: u64,
    /// Information about the proposal.
    proposal_info: ProposalInfo,
}

/// Event for a deposit.
pub struct DepositEvent {
    /// The amount deposited.
    amount: u64,
    /// The user who deposited.
    user: Identity,
}

/// Event for execution of a proposal.
pub struct ExecuteEvent {
    /// Actual acceptance percentage of approved proposal.
    acceptance_percentage: u64,
    /// The unique identifier for the proposal.
    id: u64,
    /// User who executed the event.
    user: Identity,
}

/// Event for initialization of the contract.
pub struct InitializeEvent {
    /// User who initialized the contract.
    author: Identity,
    /// AssetId of the asset used for DAO governance.
    asset: AssetId,
}

/// Event for unlocking of governance coins.
pub struct UnlockVotesEvent {
    /// The unique identifier for the proposal.
    id: u64,
    /// User who unlocks the coins.
    user: Identity,
    /// Amount of votes unlocked.
    vote_amount: u64,
}

/// Event for a vote.
pub struct VoteEvent {
    /// The unique identifier for the proposal.
    id: u64,
    /// The user who voted.
    user: Identity,
    /// The amount of votes to add to the proposal.
    vote_amount: u64,
}

/// Event for a withdrawal.
pub struct WithdrawEvent {
    /// The amount withdrawn.
    amount: u64,
    /// The user who withdrew.
    user: Identity,
}
