library;

use ::data_structures::ProposalInfo;

/// Event for creation of a new proposal.
pub struct CreateProposalEvent {
    /// The unique Identifier for the proposal.
    pub id: u64,
    /// Information about the proposal.
    pub proposal_info: ProposalInfo,
}

/// Event for a deposit.
pub struct DepositEvent {
    /// The amount deposited.
    pub amount: u64,
    /// The user who deposited.
    pub user: Identity,
}

/// Event for execution of a proposal.
pub struct ExecuteEvent {
    /// Actual acceptance percentage of approved proposal.
    pub acceptance_percentage: u64,
    /// The unique identifier for the proposal.
    pub id: u64,
    /// User who executed the event.
    pub user: Identity,
}

/// Event for initialization of the contract.
pub struct InitializeEvent {
    /// User who initialized the contract.
    pub author: Identity,
    /// AssetId of the asset used for DAO governance.
    pub asset: AssetId,
}

/// Event for unlocking of governance coins.
pub struct UnlockVotesEvent {
    /// The unique identifier for the proposal.
    pub id: u64,
    /// User who unlocks the coins.
    pub user: Identity,
    /// Amount of votes unlocked.
    pub vote_amount: u64,
}

/// Event for a vote.
pub struct VoteEvent {
    /// The unique identifier for the proposal.
    pub id: u64,
    /// The user who voted.
    pub user: Identity,
    /// The amount of votes to add to the proposal.
    pub vote_amount: u64,
}

/// Event for a withdrawal.
pub struct WithdrawEvent {
    /// The amount withdrawn.
    pub amount: u64,
    /// The user who withdrew.
    pub user: Identity,
}
