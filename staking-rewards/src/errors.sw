library errors;

use std::contract_id::ContractId;
use std::identity::Identity;

pub enum StakingRewardsError {
    StakeIncorrectToken: (),
    StakeZero: (),
    WithdrawZero: (),
}
