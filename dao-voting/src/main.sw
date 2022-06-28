contract;

dep abi;
dep data_structures;
dep errors;
dep events;

use std::{
    address::Address,
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    result::Result,
    revert::revert,
    storage::StorageMap,
    token::transfer,
};

use abi::DaoVoting;
use data_structures::{Proposal, ProposalInfo, State};
use errors::{CreationError, InitializationError, ProposalError, UserError};
use events::{CreatePropEvent, DepositEvent, ExecuteEvent, UnlockVotesEvent, VoteEvent, WithdrawEvent};

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
    u64), u64>, /// The initilization state of the contract.  Defaults to NotInitialized.
    state: State,
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
        require(storage.state == State::NotInitialized, InitializationError::CannotReinitialize);

        storage.gov_token = gov_token;
        storage.state = State::Initialized;
    }

    /// Create a new proposal
    ///
    /// # Arguments
    ///
    /// - `deadline` - the number of blocks during which a proposal can be voted on
    /// - `acceptance_percentage` - the percentage of yes votes a proposal needs to be executed
    /// - `proposal_data` - transaction data to be executed if proposal is approved
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize
    /// * When the deadline is 0
    /// * When the acceptance percentage is not greater than 0
    /// * When the acceptance percentage is not less than or equal to 100
    #[storage(read, write)]fn create_proposal(deadline: u64, acceptance_percentage: u64, proposal_transaction: Proposal) {
        require(0 < deadline, CreationError::DeadlineCannotBeZero);
        require(0 < acceptance_percentage && acceptance_percentage <= 100, CreationError::InvalidAcceptancePercentage);

        let proposal = ProposalInfo {
            yes_votes: 0,
            no_votes: 0,
            acceptance_percentage: acceptance_percentage,
            proposal_transaction: proposal_transaction,
            deadline: height() + deadline,
        };
        storage.proposals.insert(storage.proposal_count, proposal);

        let author = msg_sender().unwrap();

        log(CreatePropEvent {
            author: author, proposal_info: proposal, id: storage.proposal_count
        });

        storage.proposal_count += 1;
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
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        require(storage.gov_token == msg_asset_id(), UserError::IncorrectAssetSent);
        require(0 < msg_amount(), UserError::AmountCannotBeZero);

        let sender: Identity = msg_sender().unwrap();

        storage.balances.insert(sender, msg_amount() + storage.balances.get(sender));

        log(DepositEvent {
            amount: msg_amount(), user: sender
        });
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
        let sender: Identity = msg_sender().unwrap();

        let prev_balance = storage.balances.get(sender);
        require(amount <= prev_balance, UserError::NotEnoughAssets);

        storage.balances.insert(sender, prev_balance - amount);

        // Transfer the asset back to the user
        transfer(amount, storage.gov_token, sender);

        log(WithdrawEvent {
            amount: amount, user: sender, 
        })
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
        require(proposal_id < storage.proposal_count, UserError::InvalidId);
        require(0 < vote_amount, UserError::VoteAmountCannotBeZero);

        let mut proposal = storage.proposals.get(proposal_id);
        require(height() <= proposal.deadline, ProposalError::ProposalExpired);

        let sender: Identity = msg_sender().unwrap();
        let sender_balance = storage.balances.get(sender);

        require(vote_amount <= sender_balance, UserError::NotEnoughAssets);

        if (is_yes_vote) {
            proposal.yes_votes += vote_amount;
        } else {
            proposal.no_votes += vote_amount;
        };

        storage.balances.insert(sender, sender_balance - vote_amount);

        storage.votes.insert((sender, proposal_id), storage.votes.get((sender, proposal_id)) + vote_amount);

        storage.proposals.insert(proposal_id, proposal);

        log(VoteEvent {
            id: proposal_id, user: sender, vote_amount: vote_amount
        });
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
        require(proposal_id < storage.proposal_count, UserError::InvalidId);

        let proposal = storage.proposals.get(proposal_id);
        require(proposal.deadline < height(), ProposalError::ProposalStillActive);

        // Prevents divide by 0 error when calculating acceptance_percentage below
        if (proposal.yes_votes == 0 && proposal.no_votes == 0) {
            revert(42);
        }

        // TODO figure out how to prevent approval percentage from overflowing
        // When close to the u64 max
        // https://github.com/FuelLabs/sway-applications/issues/106
        let acceptance_percentage = proposal.yes_votes * 100 / (proposal.yes_votes + proposal.no_votes);
        require(proposal.acceptance_percentage <= acceptance_percentage, ProposalError::ApprovalPercentageNotMet);

        asm(call_data: proposal.proposal_transaction.call_data, amount: proposal.proposal_transaction.amount, asset: proposal.proposal_transaction.asset, gas: proposal.proposal_transaction.gas) {
            call call_data amount asset gas;
        }
        // Users can now convert their votes back into tokens
        log(ExecuteEvent {
            id: proposal_id, 
        });
    }

    /// Unlock governance tokens which have been locked by users who have voted on a proposal
    /// If the user had not voted on the given, expired proposal then nothing happens
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
        require(proposal_id < storage.proposal_count, UserError::InvalidId);

        let proposal = storage.proposals.get(proposal_id);
        require(proposal.deadline < height(), ProposalError::ProposalStillActive);

        let sender: Identity = msg_sender().unwrap();
        let votes = storage.votes.get((sender, proposal_id));

        storage.votes.insert((sender, proposal_id), 0);

        storage.balances.insert(sender, storage.balances.get(sender) + votes);

        log(UnlockVotesEvent {
            id: proposal_id, vote_amount: votes, 
        });
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
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        require(proposal_id < storage.proposal_count, UserError::InvalidId);
        storage.proposals.get(proposal_id)
    }
}
