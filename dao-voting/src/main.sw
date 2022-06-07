contract;

use std::{
    assert::require,
    contract_id::ContractId,
};

abi DaoVoting {
    fn constructor(gov_token: ContractId, voting_period: u64, approval_percentage: u64, proposal: b256) -> bool;
}

enum Error {
    CannotReinitialize: (),
    PeriodCannotBeZero: (),
    ApprovalPercentageCannotBeZero: (),
}

storage {
    gov_token: ContractId,
    voting_period: u64,
    approval_percentage: u64,
    proposal: b256,
    state: u64,
}

impl DaoVoting for Contract {
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
}
