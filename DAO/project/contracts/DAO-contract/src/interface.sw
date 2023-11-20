library;

use ::data_structures::{Proposal, ProposalInfo, Votes};

abi DaoVoting {
    /// Initialize the dao with the governance token.
    ///
    /// # Arguments
    ///
    /// * `gov_token`: [AssetId] - AssetId of the token used to vote on governance proposals.
    ///
    /// # Reverts
    ///
    /// * When the constructor is called more than once.
    #[storage(read, write)]
    fn constructor(gov_token: AssetId);

    /// Create a new proposal.
    ///
    /// # Arguments
    ///
    /// * `acceptance_percentage`: [u64] - the percentage of yes votes a proposal needs to be executed.
    /// * `duration`: [u64] - the number of blocks during which a proposal can be voted on.
    /// * `proposal_data`: [Proposal] - transaction data to be executed if proposal is approved.
    ///
    /// # Reverts
    ///
    /// * When the duration is 0.
    /// * When the acceptance percentage is 0.
    /// * When the acceptance percentage is greater than 100.
    #[storage(read, write)]
    fn create_proposal(
        acceptance_percentage: u64,
        duration: u64,
        proposal_transaction: Proposal,
    );

    /// Deposit governance tokens into contract.
    ///
    /// # Additional Information
    ///
    /// Update the user balance to indicate they have deposited governance tokens.
    /// A successful deposit unlocks voting functionality.
    /// Voting power is directly proportional to the amount of deposited governance tokens,
    /// That is: 1 governance token = 1 vote.
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize.
    /// * When the user deposits an asset that is not the specified governance token.
    /// * When the user does not deposit any assets.
    #[payable, storage(read, write)]
    fn deposit();

    /// Update the user balance to indicate they have withdrawn governance tokens.
    ///
    /// # Arguments
    ///
    /// * `amount`: [u64] - amount of governance tokens to withdraw from the contract.
    ///
    /// # Reverts
    ///
    /// * When the user tries to withdraw 0 from their balance.
    /// * When the user tries to withdraw more than their balance.
    #[storage(read, write)]
    fn withdraw(amount: u64);

    /// Vote on a given proposal.
    ///
    /// # Arguments
    ///
    /// * `approve`: [bool] - whether the user voted yes or no on the proposal.
    /// * `proposal_id`: [u64] - Identifier used to specifiy a proposal (0 <= proposal_id < proposal_count).
    /// * `vote_amount`: [u64] - the amount of votes to cast on the proposal.
    ///
    /// # Reverts
    ///
    /// * When the given proposal id is greater than or equal to proposal_count.
    /// * When the vote amount is 0.
    /// * When the proposal has passed its deadline.
    /// * When the vote amount is greater than the user's deposited balance.
    #[storage(read, write)]
    fn vote(approve: bool, proposal_id: u64, vote_amount: u64);

    /// Execute a given proposal.
    ///
    /// # Arguments
    ///
    /// * `proposal_id`: [u64] - Identifier used to specifiy a proposal (0 <= proposal_id < proposal_count).
    ///
    /// # Reverts
    ///
    /// * When the given proposal id is greater than or equal to proposal_count.
    /// * When the proposal has already been executed.
    /// * When the proposal is still active and being voted on.
    /// * When the proposal has not met the necessary approval percentage.
    #[storage(read, write)]
    fn execute(proposal_id: u64);

    /// Unlock governance tokens from a proposal.
    ///
    /// # Additional Information
    ///
    /// Governance tokens are locked whenever a user votes on a proposal.
    /// This is to ensure a user cannot vote twice on a proposal with the same governance token.
    /// As 1 token = 1 vote.
    /// If the user did not vote on the proposal then nothing happens.
    ///
    /// # Arguments
    ///
    /// * `proposal_id`: [u64] - Identifier used to specifiy a proposal (0 <= proposal_id < proposal_count).
    ///
    /// # Reverts
    ///
    /// * When the given proposal id is greater than or equal to proposal_count.
    /// * When the proposal is still active.
    #[storage(read, write)]
    fn unlock_votes(proposal_id: u64);
}

abi Info {
    /// Return the amount of governance tokens in this contract.
    ///
    /// # Returns
    ///
    /// * [u64] - the amount of governance tokens in this contract.
    #[storage(read)]
    fn balance() -> u64;

    /// Return the amount of governance tokens a user has in this contract.
    ///
    /// # Arguments
    ///
    /// * `user`: [Identity] - Identity to look up governance token balance in this contract.
    ///
    /// # Returns
    ///
    /// * [u64] - the amount of governance tokens a user has in this contract.
    #[storage(read)]
    fn user_balance(user: Identity) -> u64;

    /// Return the amount of votes a user has used on a proposal.
    ///
    /// # Arguments
    ///
    /// * `proposal_id`: [u64] - Identifier used to specifiy a proposal (0 <= proposal_id < proposal_count).
    /// * `user`: [Identity] - Identity to look up votes spent on a specified proposal.
    ///
    /// # Returns
    ///
    /// * [Votes] - the amount of votes a user has used on a proposal.
    #[storage(read)]
    fn user_votes(proposal_id: u64, user: Identity) -> Votes;

    /// Return proposal data for a given id.
    ///
    /// # Arguments
    ///
    /// * `proposal_id`: [u64] - Identifier used to specify a proposal (0 <= proposal_id < proposal_count).
    ///
    /// # Returns
    ///
    /// * [ProposalInfo] - proposal data for a given id.
    ///
    /// # Reverts
    ///
    /// * When the given proposal id is greater than or equal to proposal_count.
    #[storage(read)]
    fn proposal(id: u64) -> ProposalInfo;

    /// Return governance token id
    ///
    /// # Returns
    ///
    /// * [AssetId] - AssetId of the token used to vote on governance proposals.
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize
    #[storage(read)]
    fn governance_token_id() -> AssetId;

    /// Return proposal count
    ///
    /// # Returns
    ///
    /// * [u64] - the amount of proposals created.
    #[storage(read)]
    fn proposal_count() -> u64;
}
