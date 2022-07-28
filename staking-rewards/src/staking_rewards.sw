contract;

dep staking_rewards_abi;
dep staking_rewards_errors;
dep staking_rewards_events;

use std::{
    address::Address,
    chain::auth::msg_sender,
    constants::ZERO_B256,
    context::call_frames::msg_asset_id,
    context::msg_amount,
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    result::Result,
    revert::require,
    storage::StorageMap,
    token::transfer,
};

use staking_rewards_abi::StakingRewards;
use staking_rewards_errors::*;
use staking_rewards_events::*;

// Precision for staking and rewards token
//const PRECISION: u64 = 9;
const ONE: u64 = 1_000_000_000; // Can this be constant-evaluated from `PRECISION` ?

storage {
    rewards_token: ContractId = ContractId {
        value: 0x0000000000000000000000000000000000000000000000000000000000000000,
    },
    staking_token: ContractId = ContractId {
        value: 0x0000000000000000000000000000000000000000000000000000000000000000,
    },
    period_finish: u64 = 1000, // Should be start timestamp + rewards_duration
    reward_rate: u64 = 42,
    rewards_duration: u64 = 1000,
    last_update_time: u64 = 0,
    reward_per_token_stored: u64 = 0,
    user_reward_per_token_paid: StorageMap<Identity,
    u64> = StorageMap {
    },
    rewards: StorageMap<Identity,
    u64> = StorageMap {
    },
    total_supply: u64 = 0,
    balances: StorageMap<Identity,
    u64> = StorageMap {
    },
    rewards_distribution: Identity = Identity::Address(Address {
        value: 0x0000000000000000000000000000000000000000000000000000000000000000,
    },
    ), 
}

impl StakingRewards for Contract {
    // Getter functions for "public" state

    #[storage(read)]fn rewards_token() -> ContractId {
        storage.rewards_token
    }

    #[storage(read)]fn staking_token() -> ContractId {
        storage.staking_token
    }

    #[storage(read)]fn period_finish() -> u64 {
        storage.period_finish
    }

    #[storage(read)]fn reward_rate() -> u64 {
        storage.reward_rate
    }

    #[storage(read)]fn rewards_duration() -> u64 {
        storage.rewards_duration
    }

    #[storage(read)]fn last_update_time() -> u64 {
        storage.last_update_time
    }

    #[storage(read)]fn reward_per_token_stored() -> u64 {
        storage.reward_per_token_stored
    }

    #[storage(read)]fn reward_per_token_paid(account: Identity) -> u64 {
        storage.user_reward_per_token_paid.get(account)
    }

    #[storage(read)]fn rewards(account: Identity) -> u64 {
        storage.rewards.get(account)
    }

    #[storage(read)]fn rewards_distribution() -> Identity {
        storage.rewards_distribution
    }

    #[storage(read)]fn total_supply() -> u64 {
        storage.total_supply
    }

    #[storage(read)]fn balance_of(account: Identity) -> u64 {
        storage.balances.get(account)
    }

    #[storage(read)]fn last_time_reward_applicable(test_timestamp: u64) -> u64 {
        _last_time_reward_applicable(test_timestamp)
    }

    #[storage(read)]fn reward_per_token(test_timestamp: u64) -> u64 {
        _reward_per_token(test_timestamp)
    }

    #[storage(read)]fn earned(account: Identity, test_timestamp: u64) -> u64 {
        _earned(account, test_timestamp)
    }

    #[storage(read)]fn get_reward_for_duration() -> u64 {
        storage.reward_rate * storage.rewards_duration
    }

    #[storage(read, write)]fn stake(test_timestamp: u64) {
        let amount = msg_amount();
        require(amount > 0, StakingRewardsError::StakeZero);

        let asset_id = msg_asset_id();
        require(asset_id == storage.staking_token, StakingRewardsError::StakeIncorrectToken);

        let sender = msg_sender().unwrap();
        _update_reward(sender, test_timestamp);

        storage.total_supply += amount;
        storage.balances.insert(sender, storage.balances.get(sender) + amount);
        log(Staked {
            user: sender, amount: amount
        });
    }

    #[storage(read, write)]fn withdraw(amount: u64, test_timestamp: u64) {
        _withdraw(amount, test_timestamp)
    }

    #[storage(read, write)]fn get_reward(test_timestamp: u64) {
        _get_reward(test_timestamp);
    }

    #[storage(read, write)]fn exit(test_timestamp: u64) {
        _withdraw(storage.balances.get(msg_sender().unwrap()), test_timestamp);
        _get_reward(test_timestamp);
    }
}

// Non-abi (internal) functions

#[storage(read)]fn _earned(account: Identity, test_timestamp: u64) -> u64 {
    storage.balances.get(account) * (_reward_per_token(test_timestamp) - storage.user_reward_per_token_paid.get(account)) / ONE + storage.rewards.get(account)
}

#[storage(read)]fn _last_time_reward_applicable(test_timestamp: u64) -> u64 {
    // TODO (functionality): use block timestamp once implemented
    let timestamp = test_timestamp;
    let period_finish = storage.period_finish;
    // TODO (code quality): replace with a generic min function once implemented
    match timestamp < period_finish {
        true => {
            timestamp
        },
        false => {
            period_finish
        },
    }
}

#[storage(read)]fn _reward_per_token(test_timestamp: u64) -> u64 {
    let reward_per_token = storage.reward_per_token_stored;
    if (storage.total_supply == 0) {
        return reward_per_token;
    }

    reward_per_token + ((_last_time_reward_applicable(test_timestamp) - storage.last_update_time) * storage.reward_rate * ONE / storage.total_supply)
}

#[storage(read, write)]fn _get_reward(test_timestamp: u64) {
    let sender = msg_sender().unwrap();
    _update_reward(sender, test_timestamp);

    let reward = storage.rewards.get(sender);

    if (reward > 0) {
        storage.rewards.insert(sender, 0);
        transfer(reward, storage.rewards_token, sender);
        log(RewardPaid {
            user: sender, reward: reward
        });
    }
}

#[storage(read, write)]fn _withdraw(amount: u64, test_timestamp: u64) {
    require(amount > 0, StakingRewardsError::WithdrawZero);

    let sender = msg_sender().unwrap();
    _update_reward(sender, test_timestamp);

    storage.total_supply -= amount;
    storage.balances.insert(sender, storage.balances.get(sender) - amount);
    transfer(amount, storage.staking_token, sender);
    log(Withdrawn {
        user: sender, amount: amount
    });
}

#[storage(read, write)]fn _update_reward(account: Identity, test_timestamp: u64) {
    storage.reward_per_token_stored = _reward_per_token(test_timestamp);
    storage.last_update_time = _last_time_reward_applicable(test_timestamp);
    storage.rewards.insert(account, _earned(account, test_timestamp));
    storage.user_reward_per_token_paid.insert(account, storage.reward_per_token_stored);
}
