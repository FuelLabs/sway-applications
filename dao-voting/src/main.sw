contract;

use std::{
    address::Address,
    assert::require,
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
    identity::Identity,
    result::*,
    revert::revert,
    storage::StorageMap,
};

abi DaoVoting {
    fn constructor(gov_token: ContractId, voting_period: u64, approval_percentage: u64) -> bool;
    fn deposit() -> bool;
    fn get_balance() -> u64;
    fn get_user_balance(user: Identity) -> u64;
    fn get_user_votes(user: Identity) -> u64;
    fn add_proposal(proposal: b256) -> bool;
    fn get_proposal(id: u64) -> Proposal;
}

enum Error {
    CannotReinitialize: (),
    NotInitialized: (),
    NotGovernanceToken: (),
    PeriodCannotBeZero: (),
    ApprovalPercentageCannotBeZero: (),
    NoAssetsSent: (),
    InvalidId: (),
}

struct Proposal {
    approved: bool,
    expired: bool,
    data: b256,
}

storage {
    gov_token: ContractId,
    voting_period: u64,
    approval_percentage: u64,
    proposals: StorageMap<u64, Proposal>,
    proposal_count: u64,
    // The amount of governance tokens a user has deposited
    balances: StorageMap<Identity, u64>,
    // The amount of votes a user has
    votes: StorageMap<Identity, u64>,
    state: u64,
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
    fn constructor(gov_token: ContractId, voting_period: u64, approval_percentage: u64) -> bool {
        require(storage.state == 0, Error::CannotReinitialize);
        require(voting_period > 0, Error::PeriodCannotBeZero);
        require(approval_percentage > 0, Error::ApprovalPercentageCannotBeZero);

        storage.gov_token = gov_token;
        storage.voting_period = voting_period;
        storage.approval_percentage = approval_percentage;
        storage.proposal_count = 0;
        storage.state = 1;

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

    /// Add proposal to be voted on
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has not been called to initialize
    fn add_proposal(proposal_data: b256) -> bool {
        require(storage.state == 1, Error::NotInitialized);

        let proposal = Proposal {
            approved: false,
            expired: false,
            data: proposal_data,
        };
        storage.proposals.insert(storage.proposal_count, proposal);
        storage.proposal_count = storage.proposal_count + 1;
        true
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
