contract;

mod data_structures;
mod errors;
mod events;
mod interface;
mod utils;

use std::{
    auth::msg_sender,
    block::height,
    call_frames::msg_asset_id,
    context::{
        msg_amount,
        this_balance,
    },
    hash::Hash,
    token::transfer,
};

use ::data_structures::{Proposal, ProposalInfo, State, Votes};
use ::errors::{CreationError, InitializationError, ProposalError, UserError};
use ::events::{
    CreateProposalEvent,
    DepositEvent,
    ExecuteEvent,
    InitializeEvent,
    UnlockVotesEvent,
    VoteEvent,
    WithdrawEvent,
};
use ::interface::{DaoVoting, Info};
use ::utils::validate_id;

storage {
    /// The amount of governance tokens a user has deposited
    balances: StorageMap<Identity, u64> = StorageMap {},
    /// Information describing a proposal created via create_proposal(...)
    proposals: StorageMap<u64, ProposalInfo> = StorageMap {},
    /// Number of created proposals
    /// Used to check the validity of a proposal id
    /// Used as a unique identifier when creating proposals
    proposal_count: u64 = 0,
    /// The initialization state of the contract.
    state: State = State::NotInitialized,
    /// Contract Id of the governance token
    token: AssetId = AssetId::base_asset_id(),
    /// The amount of votes a user has used on a proposal
    votes: StorageMap<(Identity, u64), Votes> = StorageMap {},
}

impl DaoVoting for Contract {
    #[storage(read, write)]
    fn constructor(token: AssetId) {
        require(storage.state.read() == State::NotInitialized, InitializationError::CannotReinitialize);

        storage.token.write(token);
        storage.state.write(State::Initialized);

        log(InitializeEvent {
            author: msg_sender().unwrap(),
            token,
        })
    }

    #[storage(read, write)]
    fn create_proposal(
        acceptance_percentage: u64,
        duration: u64,
        proposal_transaction: Proposal,
    ) {
        require(0 < duration, CreationError::DurationCannotBeZero);
        require(0 < acceptance_percentage && acceptance_percentage <= 100, CreationError::InvalidAcceptancePercentage);

        let author = msg_sender().unwrap();
        let proposal = ProposalInfo::new(acceptance_percentage, author, duration, proposal_transaction);
        storage.proposals.insert(storage.proposal_count.read(), proposal);
        storage.proposal_count.write(storage.proposal_count.read() + 1);

        log(CreateProposalEvent {
            proposal_info: proposal,
            id: storage.proposal_count.read() - 1,
        });
    }

    #[payable]
    #[storage(read, write)]
    fn deposit() {
        require(storage.state.read() == State::Initialized, InitializationError::ContractNotInitialized);
        require(storage.token.read() == msg_asset_id(), UserError::IncorrectAssetSent);
        require(0 < msg_amount(), UserError::AmountCannotBeZero);

        let user = msg_sender().unwrap();

        storage.balances.insert(user, msg_amount() + storage.balances.get(user).try_read().unwrap_or(0));

        log(DepositEvent {
            amount: msg_amount(),
            user,
        });
    }

    #[storage(read, write)]
    fn withdraw(amount: u64) {
        require(0 < amount, UserError::AmountCannotBeZero);
        let user = msg_sender().unwrap();

        let prev_balance = storage.balances.get(user).try_read().unwrap_or(0);
        require(amount <= prev_balance, UserError::InsufficientBalance);

        storage.balances.insert(user, prev_balance - amount);

        // Transfer the asset back to the user
        transfer(user, storage.token.read(), amount);

        log(WithdrawEvent { amount, user })
    }

    #[storage(read, write)]
    fn vote(approve: bool, proposal_id: u64, vote_amount: u64) {
        validate_id(proposal_id, storage.proposal_count.read());
        require(0 < vote_amount, UserError::VoteAmountCannotBeZero);

        let mut proposal = storage.proposals.get(proposal_id).try_read().unwrap();
        require(proposal.deadline >= height().as_u64(), ProposalError::ProposalExpired);

        let user = msg_sender().unwrap();
        let user_balance = storage.balances.get(user).try_read().unwrap_or(0);

        require(vote_amount <= user_balance, UserError::InsufficientBalance);

        let mut votes = storage.votes.get((user, proposal_id)).try_read().unwrap_or(Votes::default());
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
            id: proposal_id,
            user,
            vote_amount,
        });
    }

    #[storage(read, write)]
    fn execute(proposal_id: u64) {
        validate_id(proposal_id, storage.proposal_count.read());

        let mut proposal = storage.proposals.get(proposal_id).try_read().unwrap();
        require(!proposal.executed, ProposalError::ProposalExecuted);
        require(proposal.deadline < height().as_u64(), ProposalError::ProposalStillActive);

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
            user: msg_sender().unwrap(),
            acceptance_percentage,
            id: proposal_id,
        })
    }

    #[storage(read, write)]
    fn unlock_votes(proposal_id: u64) {
        validate_id(proposal_id, storage.proposal_count.read());

        let proposal = storage.proposals.get(proposal_id).try_read().unwrap();
        require(proposal.deadline < height().as_u64(), ProposalError::ProposalStillActive);

        let user = msg_sender().unwrap();
        let votes = storage.votes.get((user, proposal_id)).try_read().unwrap_or(Votes::default());

        storage.votes.insert((user, proposal_id), Votes::default());

        let vote_amount = votes.yes_votes + votes.no_votes;
        storage.balances.insert(user, storage.balances.get(user).try_read().unwrap_or(0) + vote_amount);

        log(UnlockVotesEvent {
            id: proposal_id,
            user,
            vote_amount,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn balance() -> u64 {
        this_balance(storage.token.read())
    }

    #[storage(read)]
    fn user_balance(user: Identity) -> u64 {
        storage.balances.get(user).try_read().unwrap_or(0)
    }

    #[storage(read)]
    fn user_votes(proposal_id: u64, user: Identity) -> Votes {
        validate_id(proposal_id, storage.proposal_count.read());
        storage.votes.get((user, proposal_id)).try_read().unwrap_or(Votes::default())
    }

    #[storage(read)]
    fn proposal(proposal_id: u64) -> ProposalInfo {
        validate_id(proposal_id, storage.proposal_count.read());
        storage.proposals.get(proposal_id).try_read().unwrap()
    }

    #[storage(read)]
    fn governance_token_id() -> AssetId {
        require(storage.state.read() == State::Initialized, InitializationError::ContractNotInitialized);
        storage.token.read()
    }

    #[storage(read)]
    fn proposal_count() -> u64 {
        storage.proposal_count.read()
    }
}
