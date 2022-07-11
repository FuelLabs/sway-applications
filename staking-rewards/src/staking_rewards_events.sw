library staking_rewards_events;

use std::contract_id::ContractId;
use std::identity::Identity;

pub struct RewardAdded {
    reward: u64,
}

pub struct Staked {
    user: Identity,
    amount: u64,
}

pub struct Withdrawn {
    user: Identity,
    amount: u64,
}

pub struct RewardPaid {
    user: Identity,
    reward: u64,
}

pub struct RewardsDurationUpdated {
    new_duration: u64,
}

pub struct Recovered {
    token: ContractId,
    amount: u64,
}
