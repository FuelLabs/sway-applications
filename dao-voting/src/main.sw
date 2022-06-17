contract;

dep abi;
dep data_structures;
dep errors;

use std::{
    address::Address,
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    identity::Identity,
    result::*,
    revert::revert,
    storage::StorageMap,
};

use abi::DaoVoting;
use data_structures::Proposal;
use errors::Error;

storage {
    gov_token: ContractId,
    voting_period: u64,
    approval_percentage: u64,
    proposals: StorageMap<u64,
    Proposal>, proposal_count: u64,
    // The amount of governance tokens a user has deposited
    balances: StorageMap<Identity,
    u64>, // The amount of votes a user has
    votes: StorageMap<Identity,
    u64>, state: u64,
    //spent_votes: StorageMap<Identity,
    //u64>, 
}

impl DaoVoting for Contract {
    /// Initialize the dao with the governance token, voting parameters, and the proposal.
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor is called more than once
    /// - The voting period is not greater than 0
    /// - The approval percentage is not greater than 0
    fn constructor(gov_token: ContractId) -> bool {
        require(storage.state == 0, Error::CannotReinitialize);

        storage.gov_token = gov_token;
        storage.proposal_count = 0;
        storage.state = 1;

        true
    }

    /// Add proposal to be voted on
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has not been called to initialize
    fn add_proposal(voting_period: u64, approval_percentage: u64, proposal_data: b256) -> bool {
        require(storage.state == 1, Error::NotInitialized);
        require(voting_period > 0, Error::PeriodCannotBeZero);
        require(approval_percentage > 0, Error::ApprovalPercentageCannotBeZero);

        let proposal = Proposal {
            yes_votes: 0,
            no_votes: 0,
            approval_percentage: approval_percentage,
            data: proposal_data,
            end_height: height() + voting_period,
        };
        storage.proposals.insert(storage.proposal_count, proposal);
        storage.proposal_count = storage.proposal_count + 1;
        true
    }

    /// Update the user balance to indicate they have deposited governance tokens.
    /// A successful deposit unlocks voting functionality.
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has not been called to initialize
    /// - The user deposits an asset that is not the specified governance token.
    /// - The user does not deposit and assets
    fn deposit() -> bool {
        require(storage.state == 1, Error::NotInitialized);
        require(storage.gov_token == msg_asset_id(), Error::NotGovernanceToken);
        require(msg_amount() > 0, Error::NoAssetsSent);

        let result: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = result.unwrap();

        let prev_balance = storage.balances.get(sender);
        let new_balance = prev_balance + msg_amount();
        storage.balances.insert(sender, new_balance);

        true
    }

    /// Vote on a given proposal
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has not been called to initialize
    /// - The proposal id is out of range
    /// - The vote amount is 0
    /// - The vote amount is greater than the users deposited balance
    /// - The given proposal is expired
    fn vote(proposal_id: u64, vote_amount: u64, is_yes_vote: bool) -> bool {
        require(storage.state == 1, Error::NotInitialized);
        require(proposal_id < storage.proposal_count, Error::InvalidId);
        require(vote_amount > 0, Error::VoteAmountCannotBeZero);

        let result: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = result.unwrap();
        let sender_balance = storage.balances.get(sender);

        require(sender_balance >= vote_amount, Error::NotEnoughAssets);

        let mut proposal = storage.proposals.get(proposal_id);
        require(proposal.end_height >= height(), Error::ProposalExpired);

        if (is_yes_vote) {
            proposal.yes_votes = proposal.yes_votes + vote_amount;
        } else {
            proposal.no_votes = proposal.no_votes + vote_amount;
        };

        storage.balances.insert(sender, sender_balance - vote_amount);

        let votes = storage.votes.get(sender);
        storage.votes.insert(sender, votes + vote_amount);

        storage.proposals.insert(proposal_id, proposal);

        true
    }

    /// Execute a given proposal
    ///
    /// # Panics
    ///
    /// This function will panic when:
    /// - The construct has not been called to initialize
    /// - The proposal id is out of range
    /// - The proposal has not expired
    /// - The proposal has not met the necessary approval percentage
    fn execute(proposal_id: u64) -> bool {
        require(storage.state == 1, Error::NotInitialized);
        require(proposal_id < storage.proposal_count, Error::InvalidId);

        let proposal = storage.proposals.get(proposal_id);
        require(proposal.end_height > height(), Error::ProposalActive);

        let approval_percentage = proposal.yes_votes * 100 / (proposal.yes_votes + proposal.no_votes);
        require(approval_percentage >= proposal.approval_percentage, Error::ApprovalPercentageNotMet);

        // TODO execute the proposal

        // Give users back their tokens

        true
    }

    /// Return the amount of governance tokens in this contract
    fn get_balance() -> u64 {
        this_balance(storage.gov_token)
    }

    /// Return the amount of governance tokens a user has in this contract
    fn get_user_balance(user: Identity) -> u64 {
        storage.balances.get(user)
    }

    /// Return the amount of votes a user can use.
    fn get_user_votes(user: Identity) -> u64 {
        storage.votes.get(user)
    }

    /// Return proposal data for a given id
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has not been called ot initialize
    /// - The given id is out of range
    fn get_proposal(id: u64) -> Proposal {
        require(storage.state == 1, Error::NotInitialized);
        require(id < storage.proposal_count, Error::InvalidId);
        storage.proposals.get(id)
    }
}
