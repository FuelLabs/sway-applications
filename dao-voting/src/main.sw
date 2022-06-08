contract;

use std::{
    assert::require,
    chain::auth::{AuthError, msg_sender},
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    contract_id::ContractId,
};

abi DaoVoting {
    fn constructor(gov_token: ContractId, voting_period: u64, approval_percentage: u64, proposal: b256) -> bool;
    fn deposit() -> bool;
    fn get_balance() -> u64;
}

enum Error {
    CannotReinitialize: (),
    NotInitialized: (),
    NotGovernanceToken: (),
    PeriodCannotBeZero: (),
    ApprovalPercentageCannotBeZero: (),
    NoAssetsSent: (),
}

storage {
    gov_token: ContractId,
    voting_period: u64,
    approval_percentage: u64,
    proposal: b256,
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
    fn constructor(gov_token: ContractId, voting_period: u64, approval_percentage: u64, proposal: b256) -> bool {
        require(storage.state == 0, Error::CannotReinitialize);
        require(voting_period > 0, Error::PeriodCannotBeZero);
        require(approval_percentage > 0, Error::ApprovalPercentageCannotBeZero);

        storage.gov_token = gov_token;
        storage.voting_period = voting_period;
        storage.approval_percentage = approval_percentage;
        storage.proposal = proposal;
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

        true
    }

    /// Returns the amount of governance tokens in this contract
    fn get_balance() -> u64 {
        this_balance(storage.gov_token)
    }
}
