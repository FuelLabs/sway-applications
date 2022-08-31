library staking_rewards_events;

use std::contract_id::ContractId;
use std::identity::Identity;

/// Event when addtional reward tokens are added
pub struct RewardAddedEvent {
    reward: u64,
}

/// Event when someone stakes
pub struct StakedEvent {
    user: Identity,
    amount: u64,
}

/// Event when someone withdraws their staked tokens 
pub struct WithdrawnEvent {
    user: Identity,
    amount: u64,
}

/// Event when someone withdraws their reward tokens
pub struct RewardPaidEvent {
    user: Identity,
    reward: u64,
}

/// Event when the duration of staking rewards is updated
pub struct RewardsDurationUpdatedEvent {
    new_duration: u64,
}

/// Event when tokens sent to the contract address are recovered
pub struct RecoveredEvent {
    token: ContractId,
    amount: u64,
}
