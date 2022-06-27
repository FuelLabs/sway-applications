contract;

dep abi;
dep data_structures;
dep errors;
dep utils;

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
    token::transfer,
};

use abi::DaoVoting;
use data_structures::{CallData, Proposal};
use errors::Error;
use utils::sender_identity;

storage {
    // The amount of governance tokens a user has deposited
    balances: StorageMap<Identity,
    u64>, /// Contract Id of the governance token
    gov_token: ContractId,
    /// Information describing a proposal created via create_proposal(...)
    proposals: StorageMap<u64,
    Proposal>, /// Number of created proposals
    /// Used to check the validity of a proposal id
    proposal_count: u64,
    /// The amount of votes a user has used on a proposal
    votes: StorageMap<(Identity,
    u64), u64>, state: u64,
}

impl DaoVoting for Contract {
    /// Initialize the dao with the governance token, voting parameters, and the proposal.
    ///
    /// # Parameters
    ///
    /// gov_token - contract id of the token to use to vote on governance proposals
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor is called more than once
    /// - The voting period is not greater than 0
    /// - The approval percentage is not greater than 0
    #[storage(read, write)]fn constructor(gov_token: ContractId) -> bool {
        require(storage.state == 0, Error::CannotReinitialize);

        storage.gov_token = gov_token;
        storage.proposal_count = 0;
        storage.state = 1;

        true
    }

    /// Add proposal to be voted on
    ///
    /// # Parameters
    ///
    /// voting_period - the number of blocks during which a proposal can be voted on
    /// approval_percentage - the percentage of yes votes a proposal needs to be executed
    /// proposal_data - transaction data to be executed if proposal is approved
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has not been called to initialize
    /// - The voting period is 0
    /// - The approval percentage is 0
    /// - The approval percentage is above 100
    #[storage(read, write)]fn add_proposal(voting_period: u64, approval_percentage: u64, proposal_data: CallData) -> bool {
        require(storage.state == 1, Error::NotInitialized);
        require(voting_period > 0, Error::PeriodCannotBeZero);
        require(approval_percentage > 0, Error::ApprovalPercentageCannotBeZero);
        require(approval_percentage <= 100, Error::ApprovalPercentageCannotBeAboveHundred);

        let proposal = Proposal {
            yes_votes: 0,
            no_votes: 0,
            approval_percentage: approval_percentage,
            call_data: proposal_data,
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
    #[storage(read, write)]fn deposit() -> bool {
        require(storage.state == 1, Error::NotInitialized);
        require(storage.gov_token == msg_asset_id(), Error::NotGovernanceToken);
        require(msg_amount() > 0, Error::NoAssetsSent);

        let sender: Identity = sender_identity();

        let prev_balance = storage.balances.get(sender);
        let new_balance = prev_balance + msg_amount();
        storage.balances.insert(sender, new_balance);

        true
    }

    /// Update the user balance to indicate they have withdrawn governance tokens
    ///
    /// # Parameters
    ///
    /// amount - amount of governance tokens to withdraw from the contract
    ///
    /// # Panics
    ///
    /// This functions will panic when:
    /// - The constructor has not been called to initalize
    /// - The user tries to withdraw more than their balance
    #[storage(read, write)]fn withdraw(amount: u64) -> bool {
        require(storage.state == 1, Error::NotInitialized);

        let sender: Identity = sender_identity();

        let prev_balance = storage.balances.get(sender);
        require(prev_balance >= amount, Error::NotEnoughAssets);

        let new_balance = prev_balance - amount;
        storage.balances.insert(sender, new_balance);

        // Transfer the asset back to the user
        transfer(amount, storage.gov_token, sender);

        true
    }

    /// Vote on a given proposal
    ///
    /// # Parameters
    ///
    /// proposal_id - proposal to vote on
    /// vote_amount - amount of votes to use on proposal
    /// is_yes_vote - determines if you vote yes or no on the proposal
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has not been called to initialize
    /// - The proposal id is out of range
    /// - The vote amount is 0
    /// - The vote amount is greater than the users deposited balance
    /// - The given proposal is expired
    #[storage(read, write)]fn vote(proposal_id: u64, vote_amount: u64, is_yes_vote: bool) -> bool {
        require(storage.state == 1, Error::NotInitialized);
        require(proposal_id < storage.proposal_count, Error::InvalidId);
        require(vote_amount > 0, Error::VoteAmountCannotBeZero);

        let sender: Identity = sender_identity();
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

        let votes = storage.votes.get((sender, proposal_id));
        storage.votes.insert((sender, proposal_id), votes + vote_amount);

        storage.proposals.insert(proposal_id, proposal);

        true
    }

    /// Execute a given proposal
    ///
    /// # Parameters
    ///
    /// proposal_id - proposal to execute
    ///
    /// # Panics
    ///
    /// This function will panic when:
    /// - The construct has not been called to initialize
    /// - The proposal id is out of range
    /// - The proposal has not expired
    /// - The proposal has not met the necessary approval percentage
    #[storage(read, write)]fn execute(proposal_id: u64) -> bool {
        require(storage.state == 1, Error::NotInitialized);
        require(proposal_id < storage.proposal_count, Error::InvalidId);

        let proposal = storage.proposals.get(proposal_id);
        require(proposal.end_height < height(), Error::ProposalActive);

        // TODO figure out how to prevent approval percentage from overflowing
        // When close to the u64 max
        // https://github.com/FuelLabs/sway-applications/issues/106
        let approval_percentage = proposal.yes_votes * 100 / (proposal.yes_votes + proposal.no_votes);
        require(approval_percentage >= proposal.approval_percentage, Error::ApprovalPercentageNotMet);

        asm(rA: proposal.call_data.memory_address, rB: proposal.call_data.num_coins_to_forward, rC: proposal.call_data.asset_id_of_coins_to_forward, rD: proposal.call_data.amount_of_gas_to_forward) {
            call rA rB rC rD;
        }

        // Users can now convert their votes back into tokens

        true
    }

    /// Unlock tokens used to vote on proposals to allow the user to withdraw
    /// If the user had not voted in the given expired proposal, nothing happens
    ///
    /// # Parameters
    /// proposal_id - proposal to turn user votes back into governance tokens
    ///
    /// # Panics
    ///
    /// This function will panic when:
    /// - The constructor has not ben called to initialize
    /// - The proposal id is invalid
    /// - The proposal is still active
    #[storage(read, write)]fn convert_votes_to_tokens(proposal_id: u64) {
        require(storage.state == 1, Error::NotInitialized);
        require(proposal_id < storage.proposal_count, Error::InvalidId);

        let proposal = storage.proposals.get(proposal_id);
        require(proposal.end_height < height(), Error::ProposalActive);

        let sender: Identity = sender_identity();
        let votes = storage.votes.get((sender, proposal_id));

        storage.votes.insert((sender, proposal_id), 0);

        let balance = storage.balances.get(sender);
        storage.balances.insert(sender, balance + votes);
    }

    /// Return the amount of governance tokens in this contract
    #[storage(read)]fn balance() -> u64 {
        this_balance(storage.gov_token)
    }

    /// Return the amount of governance tokens a user has in this contract
    ///
    /// # Parameters
    /// user - user of which to get internal balance of governance tokens
    #[storage(read)]fn user_balance(user: Identity) -> u64 {
        storage.balances.get(user)
    }

    /// Return the amount of votes a user has used on a proposal
    ///
    /// # Parameters
    /// user - user of which to get votes spent on a proposal
    /// proposal_id - proposal of which to get votes spent by user
    #[storage(read)]fn user_votes(user: Identity, proposal_id: u64) -> u64 {
        require(proposal_id < storage.proposal_count, Error::InvalidId);
        storage.votes.get((user, proposal_id))
    }

    /// Return proposal data for a given id
    ///
    /// # Parameters
    /// proposal_id - id of proposal to get
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has not been called ot initialize
    /// - The given id is out of range
    #[storage(read)]fn proposal(proposal_id: u64) -> Proposal {
        require(storage.state == 1, Error::NotInitialized);
        require(proposal_id < storage.proposal_count, Error::InvalidId);
        storage.proposals.get(proposal_id)
    }
}
