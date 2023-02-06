library data_structures;

use std::contract_id::ContractId;
use std::identity::Identity;

/// Event when tokens sent to the contract address are recovered
pub struct RecoveredEvent {
    /// The asset ID of the recovered token
    token: ContractId,
    /// The amount of tokens recovered
    amount: u64,
}

/// Event when additional reward tokens are added
pub struct RewardAddedEvent {
    /// The amount of additional tokens added
    reward: u64,
}

/// Event when someone withdraws their reward tokens
pub struct RewardPaidEvent {
    /// The Idenitity of the person who withdrew
    user: Identity,
    /// The amount of reward tokens withdrawn
    reward: u64,
}

/// Event when the duration of staking rewards is updated
pub struct RewardsDurationUpdatedEvent {
    /// The new duration of the staking rewards
    new_duration: u64,
}

/// Event when someone stakes
pub struct StakedEvent {
    /// The Identity of the person who staked
    user: Identity,
    /// The amount the person staked
    amount: u64,
}

/// Event when someone withdraws their staked tokens
pub struct WithdrawnEvent {
    /// The Identity of the person who withdrew
    user: Identity,
    /// The amount of tokens the person withdrew
    amount: u64,
}
