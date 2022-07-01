contract;

dep dao_voting_abi;
dep data_structures;
dep errors;
dep events;
dep utils;

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

use dao_voting_abi::DaoVoting;
use data_structures::{Proposal, ProposalInfo, State, Votes};
use errors::{CreationError, InitializationError, ProposalError, UserError};
use events::{
    CreatePropEvent,
    DepositEvent,
    ExecuteEvent,
    InitializeEvent,
    UnlockVotesEvent,
    VoteEvent,
    WithdrawEvent,
};

use utils::validate_id;

storage {
    /// The amount of governance tokens a user has deposited
    balances: StorageMap<Identity,
    u64>, /// Information describing a proposal created via create_proposal(...)
    proposals: StorageMap<u64,
    ProposalInfo>, /// Number of created proposals
    /// Used to check the validity of a proposal id
    /// Used as a unique identifier when creating proposals
    proposal_count: u64,
    /// The initilization state of the contract.
    state: State,
    /// Contract Id of the governance token
    token: ContractId,
    /// The amount of votes a user has used on a proposal
    votes: StorageMap<(Identity,
    u64), Votes>, 
}

impl DaoVoting for Contract {
    /// Initialize the dao with the governance token, voting parameters, and the proposal.
    ///
    /// # Arguments
    ///
    /// - `token` - contract id of the token used to vote on governance proposals
    ///
    /// # Reverts
    ///
    /// * When the constructor is called more than once
    #[storage(read, write)]fn constructor(token: ContractId) {
        require(storage.state == State::NotInitialized, InitializationError::CannotReinitialize);

        storage.token = token;
        storage.state = State::Initialized;

        log(InitializeEvent {
            author: msg_sender().unwrap(), token
        })
    }

    /// Create a new proposal
    ///
    /// # Arguments
    ///
    /// - `acceptance_percentage` - the percentage of yes votes a proposal needs to be executed
    /// - `duration` - the number of blocks during which a proposal can be voted on
    /// - `proposal_data` - transaction data to be executed if proposal is approved
    ///
    /// # Reverts
    ///
    /// * When the duration is 0
    /// * When the acceptance percentage is 0
    /// * When the acceptance percentage is greater than 100
    #[storage(read, write)]fn create_proposal(acceptance_percentage: u64, duration: u64, proposal_transaction: Proposal) {
        require(0 < duration, CreationError::DurationCannotBeZero);
        require(0 < acceptance_percentage && acceptance_percentage <= 100, CreationError::InvalidAcceptancePercentage);

        let author = msg_sender().unwrap();
        let proposal = ~ProposalInfo::new(acceptance_percentage, author, duration, proposal_transaction);
        storage.proposals.insert(storage.proposal_count, proposal);
        storage.proposal_count += 1;

        log(CreatePropEvent {
            proposal_info: proposal, id: storage.proposal_count - 1
        });
    }

    /// Deposit governance tokens into contract
    ///
    /// Update the user balance to indicate they have deposited governance tokens.
    /// A successful deposit unlocks voting functionality.
    /// Voting power is directly proportional to the amount of deposited governance tokens
    /// That is: 1 governance token = 1 vote
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize
    /// * When the user deposits an asset that is not the specified governance token.
    /// * When the user does not deposit any assets
    #[storage(read, write)]fn deposit() {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        require(storage.token == msg_asset_id(), UserError::IncorrectAssetSent);
        require(0 < msg_amount(), UserError::AmountCannotBeZero);

        let user = msg_sender().unwrap();

        storage.balances.insert(user, msg_amount() + storage.balances.get(user));

        log(DepositEvent {
            amount: msg_amount(), user
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
    /// * When the user tries to withdraw 0 from their balance
    /// * When the user tries to withdraw more than their balance
    #[storage(read, write)]fn withdraw(amount: u64) {
        require(0 < amount, UserError::AmountCannotBeZero);
        let user: Identity = msg_sender().unwrap();

        let prev_balance = storage.balances.get(user);
        require(amount <= prev_balance, UserError::InsufficientBalance);

        storage.balances.insert(user, prev_balance - amount);

        // Transfer the asset back to the user
        transfer(amount, storage.token, user);

        log(WithdrawEvent {
            amount, user, 
        })
    }

    /// Vote on a given proposal
    ///
    /// # Arguments
    ///
    /// - `approve` - whether the user voted yes or no on the proposal
    /// - `proposal_id` - Identifier used to specifiy a proposal (0 <= proposal_id < proposal_count)
    /// - `vote_amount` - the amount of votes to cast on the proposal
    ///
    /// # Reverts
    ///
    /// * When the given proposal id is greater than or equal to proposal_count
    /// * When the vote amount is 0
    /// * When the proposal has passed its deadline
    /// * When the vote amount is greater than the user's deposited balance
    #[storage(read, write)]fn vote(approve: bool, proposal_id: u64, vote_amount: u64) {
        validate_id(proposal_id, storage.proposal_count);
        require(0 < vote_amount, UserError::VoteAmountCannotBeZero);

        let mut proposal = storage.proposals.get(proposal_id);
        require(height() <= proposal.deadline, ProposalError::ProposalExpired);

        let user = msg_sender().unwrap();
        let user_balance = storage.balances.get(user);

        require(vote_amount <= user_balance, UserError::InsufficientBalance);

        let mut votes = storage.votes.get((user, proposal_id));
        if approve {
            proposal.yes_votes += vote_amount;
            votes.yes_votes += vote_amount;
        } else {
            proposal.no_votes += vote_amount;
            votes.no_votes += vote_amount;
        };

        storage.balances.insert(user, user_balance - vote_amount);
        storage.votes.insert((user, proposal_id), votes);
        storage.proposals.insert(proposal_id, proposal);

        log(VoteEvent {
            id: proposal_id, user, vote_amount
        });
    }

    /// Execute a given proposal
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - Identifier used to specifiy a proposal (0 <= proposal_id < proposal_count)
    ///
    /// # Reverts
    ///
    /// * When the given proposal id is greater than or equal to proposal_count
    /// * When the proposal has already been executed
    /// * When the proposal is still active and being voted on
    /// * When the proposal has not met the necessary approval percentage
    #[storage(read, write)]fn execute(proposal_id: u64) {
        validate_id(proposal_id, storage.proposal_count);

        let mut proposal = storage.proposals.get(proposal_id);
        require(!proposal.executed, ProposalError::ProposalExecuted);
        require(proposal.deadline < height(), ProposalError::ProposalStillActive);

        // TODO figure out how to prevent approval percentage from overflowing
        // When close to the u64 max
        // https://github.com/FuelLabs/sway-applications/issues/106
        let acceptance_percentage = proposal.yes_votes * 100 / (proposal.yes_votes + proposal.no_votes);
        require(proposal.acceptance_percentage <= acceptance_percentage, ProposalError::InsufficientApprovals);

        proposal.executed = true;
        storage.proposals.insert(proposal_id, proposal);

        asm(call_data: proposal.proposal_transaction.call_data, amount: proposal.proposal_transaction.amount, asset: proposal.proposal_transaction.asset, gas: proposal.proposal_transaction.gas) {
            call call_data amount asset gas;
        }

        // Users can now convert their votes back into tokens
        log(ExecuteEvent {
            user: msg_sender().unwrap(), acceptance_percentage, id: proposal_id, 
        })
    }

    /// Unlock governance tokens from a proposal
    ///
    /// Governance tokens are locked whenever a user votes on a proposal.
    /// This is to ensure a user cannot vote twice on a proposal with the same governance token.
    /// As 1 token = 1 vote.
    /// If the user did not vote on the proposal then nothing happens
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - Identifier used to specifiy a proposal (0 <= proposal_id < proposal_count)
    ///
    /// # Reverts
    ///
    /// * When the given proposal id is greater than or equal to proposal_count
    /// * When the proposal is still active
    #[storage(read, write)]fn unlock_votes(proposal_id: u64) {
        validate_id(proposal_id, storage.proposal_count);

        let proposal = storage.proposals.get(proposal_id);
        require(proposal.deadline < height(), ProposalError::ProposalStillActive);

        let user: Identity = msg_sender().unwrap();
        let votes = storage.votes.get((user, proposal_id));

        storage.votes.insert((user, proposal_id), Votes {
            no_votes: 0, yes_votes: 0
        });

        let vote_amount = votes.yes_votes + votes.no_votes;
        storage.balances.insert(user, storage.balances.get(user) + vote_amount);

        log(UnlockVotesEvent {
            id: proposal_id, user, vote_amount, 
        });
    }

    /// Return the amount of governance tokens in this contract
    #[storage(read)]fn balance() -> u64 {
        this_balance(storage.token)
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
    /// - `proposal_id` - Identifier used to specifiy a proposal (0 <= proposal_id < proposal_count)
    /// - `user` - Identity to look up votes spent on a specified proposal
    #[storage(read)]fn user_votes(proposal_id: u64, user: Identity) -> Votes {
        validate_id(proposal_id, storage.proposal_count);
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
    /// * When the given proposal id is greater than or equal to proposal_count
    #[storage(read)]fn proposal(proposal_id: u64) -> ProposalInfo {
        validate_id(proposal_id, storage.proposal_count);
        storage.proposals.get(proposal_id)
    }

    /// Return governance token id
    ///
    /// # Reverts
    ///
    /// * When the constructor has not been called to initialize
    #[storage(read)] fn governance_token_id() -> ContractId {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        storage.token
    }

    /// Return proposal count
    #[storage(read)] fn proposal_count() -> u64 {
        storage.proposal_count
    }
}
