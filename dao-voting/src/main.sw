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
    token::transfer,
};

use abi::DaoVoting;
use data_structures::{Proposal, ProposalInfo};
use errors::{CreationError, InitializationError, ProposalError, UserError};

storage {
    // The amount of governance tokens a user has deposited
    balances: StorageMap<Identity,
    u64>, /// Contract Id of the governance token
    gov_token: ContractId,
    /// Information describing a proposal created via create_proposal(...)
    proposals: StorageMap<u64,
    ProposalInfo>, /// Number of created proposals
    /// Used to check the validity of a proposal id
    proposal_count: u64,
    /// The amount of votes a user has used on a proposal
    votes: StorageMap<(Identity,
    u64), u64>, state: u64,
}

impl DaoVoting for Contract {
    /// Initialize the dao with the governance token, voting parameters, and the proposal.
    ///
    /// # Arguments
    ///
    /// - `gov_token` - contract id of the token used to vote on governance proposals
    ///
    /// # Reverts
    ///
    /// * When the constructor is called more than once
    #[storage(read, write)]fn constructor(gov_token: ContractId) {
        require(storage.state == 0, InitializationError::CannotReinitialize);

        storage.gov_token = gov_token;
        storage.state = 1;
    }

    /// Create a new proposal
    ///
    /// # Arguments
    ///
    /// - `end_height` - the number of blocks during which a proposal can be voted on
    /// - `acceptance_percentage` - the percentage of yes votes a proposal needs to be executed
    /// - `proposal_data` - transaction data to be executed if proposal is approved
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize
    /// * When the end height is 0
    /// * When the acceptance percentage is 0
    /// * When the acceptance percentage is above 100
    #[storage(read, write)]fn create_proposal(end_height: u64, acceptance_percentage: u64, proposal_data: Proposal) {
        require(storage.state == 1, InitializationError::ContractNotInitialized);
        require(0 < end_height, CreationError::EndHeightCannotBeZero);
        require(0 < acceptance_percentage, CreationError::AcceptancePercentageCannotBeZero);
        require(acceptance_percentage <= 100, CreationError::AcceptancePercentageCannotBeAboveOneHundred);

        let proposal = ProposalInfo {
            yes_votes: 0,
            no_votes: 0,
            acceptance_percentage: acceptance_percentage,
            call_data: proposal_data,
            end_height: height() + end_height,
        };
        storage.proposals.insert(storage.proposal_count, proposal);
        storage.proposal_count = storage.proposal_count + 1;
    }

    /// Update the user balance to indicate they have deposited governance tokens.
    /// A successful deposit unlocks voting functionality.
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize
    /// * When the user deposits an asset that is not the specified governance token.
    /// * When the user does not deposit any assets
    #[storage(read, write)]fn deposit() {
        require(storage.state == 1, InitializationError::ContractNotInitialized);
        require(storage.gov_token == msg_asset_id(), UserError::IncorrectAssetSent);
        require(0 < msg_amount(), UserError::AmountCannotBeZero);

        let sender: Identity = msg_sender().unwrap();

        storage.balances.insert(sender, msg_amount() + storage.balances.get(sender));
    }

    /// Update the user balance to indicate they have withdrawn governance tokens
    ///
    /// # Arguments
    ///
    /// - `amount` - amount of governance tokens to withdraw from the contract
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initalize
    /// * When the user tries to withdraw more than their balance
    #[storage(read, write)]fn withdraw(amount: u64) {
        require(storage.state == 1, InitializationError::ContractNotInitialized);

        let sender: Identity = msg_sender().unwrap();

        let prev_balance = storage.balances.get(sender);
        require(amount <= prev_balance, UserError::NotEnoughAssets);

        storage.balances.insert(sender, prev_balance - amount);

        // Transfer the asset back to the user
        transfer(amount, storage.gov_token, sender);
    }

    /// Vote on a given proposal
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - proposal to vote on
    /// - `vote_amount` - amount of votes to use on proposal
    /// - `is_yes_vote` - determines if you vote yes or no on the proposal
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize
    /// * When the proposal id is out of range
    /// * When the vote amount is 0
    /// * When the vote amount is greater than the users deposited balance
    /// * When the given proposal is expired
    #[storage(read, write)]fn vote(proposal_id: u64, vote_amount: u64, is_yes_vote: bool) {
        require(storage.state == 1, InitializationError::ContractNotInitialized);
        require(proposal_id < storage.proposal_count, UserError::InvalidId);
        require(0 < vote_amount, UserError::VoteAmountCannotBeZero);

        let sender: Identity = msg_sender().unwrap();
        let sender_balance = storage.balances.get(sender);

        require(vote_amount <= sender_balance, UserError::NotEnoughAssets);

        let mut proposal = storage.proposals.get(proposal_id);
        require(height() <= proposal.end_height, ProposalError::ProposalExpired);

        if (is_yes_vote) {
            proposal.yes_votes += vote_amount;
        } else {
            proposal.no_votes += vote_amount;
        };

        storage.balances.insert(sender, sender_balance - vote_amount);

        storage.votes.insert((sender, proposal_id), storage.votes.get((sender, proposal_id)) + vote_amount);

        storage.proposals.insert(proposal_id, proposal);
    }

    /// Execute a given proposal
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - proposal to execute
    ///
    /// # Reverts
    ///
    /// * When the construct has not been called to initialize
    /// * When the proposal id is out of range
    /// * When the proposal has not expired
    /// * When the proposal has not met the necessary approval percentage
    #[storage(read, write)]fn execute(proposal_id: u64) {
        require(storage.state == 1, InitializationError::ContractNotInitialized);
        require(proposal_id < storage.proposal_count, UserError::InvalidId);

        let proposal = storage.proposals.get(proposal_id);
        require(proposal.end_height < height(), ProposalError::ProposalStillActive);

        // TODO figure out how to prevent approval percentage from overflowing
        // When close to the u64 max
        // https://github.com/FuelLabs/sway-applications/issues/106
        let acceptance_percentage = proposal.yes_votes * 100 / (proposal.yes_votes + proposal.no_votes);
        require(proposal.acceptance_percentage <= acceptance_percentage, ProposalError::ApprovalPercentageNotMet);

        asm(rA: proposal.call_data.memory_address, rB: proposal.call_data.num_coins_to_forward, rC: proposal.call_data.asset_id_of_coins_to_forward, rD: proposal.call_data.amount_of_gas_to_forward) {
            call rA rB rC rD;
        }
        // Users can now convert their votes back into tokens
    }

    /// Unlock tokens used to vote on proposals to allow the user to withdraw
    /// If the user had not voted in the given expired proposal, nothing happens
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - proposal to turn user votes back into governance tokens
    ///
    /// # Reverts
    ///
    /// * When the constructor has not ben called to initialize
    /// * When the proposal id is invalid
    /// * When the proposal is still active
    #[storage(read, write)]fn unlock_votes(proposal_id: u64) {
        require(storage.state == 1, InitializationError::ContractNotInitialized);
        require(proposal_id < storage.proposal_count, UserError::InvalidId);

        let proposal = storage.proposals.get(proposal_id);
        require(proposal.end_height < height(), ProposalError::ProposalStillActive);

        let sender: Identity = msg_sender().unwrap();
        let votes = storage.votes.get((sender, proposal_id));

        storage.votes.insert((sender, proposal_id), 0);

        storage.balances.insert(sender, storage.balances.get(sender) + votes);
    }

    /// Return the amount of governance tokens in this contract
    #[storage(read)]fn balance() -> u64 {
        this_balance(storage.gov_token)
    }

    /// Return the amount of governance tokens a user has in this contract
    ///
    /// # Arguments
    ///
    /// - `user` - Identity to look up governance token balance in this contract.
    #[storage(read)]fn user_balance(user: Identity) -> u64 {
        storage.balances.get(user)
    }

    /// Return the amount of votes a user has used on a proposal
    ///
    /// # Arguments
    ///
    /// - `user` - Identity to look up votes spent on a specified proposal
    /// - `proposal_id` - Identifier used to specifiy a proposal (0 <= proposal_id < proposal_count)
    #[storage(read)]fn user_votes(user: Identity, proposal_id: u64) -> u64 {
        require(proposal_id < storage.proposal_count, UserError::InvalidId);
        storage.votes.get((user, proposal_id))
    }

    /// Return proposal data for a given id
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - Identifier used to specify a proposal (0 <= proposal_id < proposal_count)
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called ot initialize
    /// * When the given proposal id is out of range
    #[storage(read)]fn proposal(proposal_id: u64) -> ProposalInfo {
        require(storage.state == 1, InitializationError::ContractNotInitialized);
        require(proposal_id < storage.proposal_count, UserError::InvalidId);
        storage.proposals.get(proposal_id)
    }
}
