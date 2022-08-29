library staking_rewards_events;

use std::contract_id::ContractId;
use std::identity::Identity;

pub struct RewardAddedEvent {
    reward: u64,
}

pub struct StakedEvent {
    user: Identity,
    amount: u64,
}

pub struct WithdrawnEvent {
    user: Identity,
    amount: u64,
}

pub struct RewardPaidEvent {
    user: Identity,
    reward: u64,
}

pub struct RewardsDurationUpdatedEvent {
    new_duration: u64,
}

pub struct RecoveredEvent {
    token: ContractId,
    amount: u64,
}
