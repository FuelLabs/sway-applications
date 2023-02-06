contract;

dep interface;
dep errors;
dep data_structures;

use data_structures::*;
use errors::StakingRewardsError;
use interface::StakingRewards;
use std::{
    address::Address,
    auth::msg_sender,
    block::timestamp,
    call_frames::msg_asset_id,
    constants::ZERO_B256,
    context::{
        msg_amount,
        this_balance,
    },
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    result::Result,
    revert::require,
    storage::StorageMap,
    token::transfer,
};

// Precision for staking and rewards token
const ONE: u64 = 1_000_000_000; // Can this be constant-evaluated from `PRECISION` ?
storage {
    balances: StorageMap<Identity, u64> = StorageMap {},
    last_update_time: u64 = 0,
    owner: Identity = Identity::Address(Address { value: owner }),
    period_finish: u64 = 1000, // Should be start timestamp + rewards_duration
    rewards: StorageMap<Identity, u64> = StorageMap {},
    rewards_distribution: Identity = Identity::Address(Address {
        value: rewards_distribution,
    }),
    rewards_duration: u64 = 1000,
    rewards_token: ContractId = ContractId {
        value: rewards_token_b256,
    },
    reward_rate: u64 = 42,
    reward_per_token_stored: u64 = 0,
    staking_token: ContractId = ContractId {
        value: staking_token_b256,
    },
    total_supply: u64 = 0,
    user_reward_per_token_paid: StorageMap<Identity, u64> = StorageMap {},
}

impl StakingRewards for Contract {
    #[storage(read)]
    fn balance_of(account: Identity) -> u64 {
        storage.balances.get(account)
    }

    #[storage(read)]
    fn earned(account: Identity) -> u64 {
        _earned(account)
    }

    #[storage(read, write)]
    fn exit() {
        let sender = msg_sender().unwrap();
        let amount = storage.balances.get(sender);
        _withdraw(amount);
        log(WithdrawnEvent {
            user: sender,
            amount,
        });
        _update_reward(sender);

        let reward = storage.rewards.get(sender);

        if (reward > 0) {
            storage.rewards.insert(sender, 0);
            transfer(reward, storage.rewards_token, sender);
            log(RewardPaidEvent {
                user: sender,
                reward,
            });
        }
    }

    #[storage(read, write)]
    fn get_reward() {
        let sender = msg_sender().unwrap();
        _update_reward(sender);

        let reward = storage.rewards.get(sender);

        if (reward > 0) {
            storage.rewards.insert(sender, 0);
            transfer(reward, storage.rewards_token, sender);
            log(RewardPaidEvent {
                user: sender,
                reward,
            });
        }
    }

    #[storage(read)]
    fn get_reward_for_duration() -> u64 {
        storage.reward_rate * storage.rewards_duration
    }

    #[storage(read)]
    fn last_time_reward_applicable() -> u64 {
        _last_time_reward_applicable()
    }

    #[storage(read)]
    fn last_update_time() -> u64 {
        storage.last_update_time
    }

    #[storage(read, write)]
    fn notify_reward_amount(reward: u64) {
        let sender = msg_sender().unwrap();
        let ts = timestamp();
        _update_reward(sender);

        require(sender == storage.rewards_distribution, StakingRewardsError::CallerIsNotRewardsDistributionContract);

        if ts >= storage.period_finish {
            storage.reward_rate = reward / storage.rewards_duration;
        } else {
            let remaining = storage.period_finish - ts;
            let leftover = remaining * storage.reward_rate;
            storage.reward_rate = (reward + leftover) / storage.rewards_duration;
        }

        // Ensure the provided reward amount is not more than the balance in the contract.
        // This keeps the reward rate in the right range, preventing overflows due to
        // very high values of rewardRate in the earned and rewardsPerToken functions;
        // Reward + leftover must be less than 2^256 / 10^18 to avoid overflow.
        let balance = this_balance(storage.rewards_token);
        require(storage.reward_rate <= balance / storage.rewards_duration, StakingRewardsError::ProvidedRewardTooHigh);

        storage.last_update_time = ts;
        storage.period_finish = ts + storage.rewards_duration;
        log(RewardAddedEvent { reward });
    }

    #[storage(read)]
    fn owner() -> Identity {
        storage.owner
    }

    #[storage(read)]
    fn period_finish() -> u64 {
        storage.period_finish
    }

    // Added to support recovering LP Rewards from other systems such as BAL to be distributed to holders
    #[storage(read)]
    fn recover_tokens(asset_id: ContractId, amount: u64) {
        require(msg_sender().unwrap() == storage.owner, StakingRewardsError::SenderNotOwner);

        require(asset_id != storage.staking_token, StakingRewardsError::CannotWithdrawTheStakingToken);
        transfer(amount, asset_id, storage.owner);

        log(RecoveredEvent {
            token: asset_id,
            amount,
        });
    }

    #[storage(read)]
    fn reward_per_token() -> u64 {
        _reward_per_token()
    }

    #[storage(read)]
    fn reward_per_token_stored() -> u64 {
        storage.reward_per_token_stored
    }

    #[storage(read)]
    fn reward_per_token_paid(account: Identity) -> u64 {
        storage.user_reward_per_token_paid.get(account)
    }

    #[storage(read)]
    fn reward_rate() -> u64 {
        storage.reward_rate
    }

    #[storage(read)]
    fn rewards(account: Identity) -> u64 {
        storage.rewards.get(account)
    }

    #[storage(read)]
    fn rewards_distribution() -> Identity {
        storage.rewards_distribution
    }

    #[storage(read)]
    fn rewards_duration() -> u64 {
        storage.rewards_duration
    }

    #[storage(read)]
    fn rewards_token() -> ContractId {
        storage.rewards_token
    }

    #[storage(read, write)]
    fn set_rewards_duration(rewards_duration: u64) {
        require(msg_sender().unwrap() == storage.owner, StakingRewardsError::SenderNotOwner);

        require(timestamp() > storage.period_finish, StakingRewardsError::PreviousRewardsPeriodMustBeCompleteBeforeChangingTheDurationForTheNewPeriod);
        storage.rewards_duration = rewards_duration;
        log(RewardsDurationUpdatedEvent {
            new_duration: rewards_duration,
        });
    }

    #[storage(read, write)]
    fn stake() {
        let amount = msg_amount();
        require(amount > 0, StakingRewardsError::StakeZero);

        require(msg_asset_id() == storage.staking_token, StakingRewardsError::StakeIncorrectToken);

        let user = msg_sender().unwrap();
        _update_reward(user);

        storage.total_supply += amount;
        storage.balances.insert(user, storage.balances.get(user) + amount);
        log(StakedEvent { user, amount });
    }

    #[storage(read)]
    fn staking_token() -> ContractId {
        storage.staking_token
    }

    #[storage(read)]
    fn total_supply() -> u64 {
        storage.total_supply
    }

    #[storage(read, write)]
    fn withdraw(amount: u64) {
        _withdraw(amount);
        log(WithdrawnEvent {
            user: msg_sender().unwrap(),
            amount,
        });
    }
}

// Non-abi (internal) functions
#[storage(read)]
fn _earned(account: Identity) -> u64 {
    storage.balances.get(account) * (_reward_per_token() - storage.user_reward_per_token_paid.get(account)) / ONE + storage.rewards.get(account)
}

#[storage(read)]
fn _last_time_reward_applicable() -> u64 {
    let ts = timestamp();
    let period_finish = storage.period_finish;
    // TODO (code quality): replace with a generic min function once implemented
    match ts < period_finish {
        true => {
            ts
        },
        false => {
            period_finish
        },
    }
}

#[storage(read)]
fn _reward_per_token() -> u64 {
    let reward_per_token = storage.reward_per_token_stored;

    match storage.total_supply {
        0 => reward_per_token,
        _ => reward_per_token + ((_last_time_reward_applicable() - storage.last_update_time) * storage.reward_rate * ONE / storage.total_supply),
    }
}

#[storage(read, write)]
fn _get_reward() {}

#[storage(read, write)]
fn _withdraw(amount: u64) {
    require(amount > 0, StakingRewardsError::WithdrawZero);

    let sender = msg_sender().unwrap();
    _update_reward(sender);

    storage.total_supply -= amount;
    storage.balances.insert(sender, storage.balances.get(sender) - amount);
    transfer(amount, storage.staking_token, sender);
}

#[storage(read, write)]
fn _update_reward(account: Identity) {
    storage.reward_per_token_stored = _reward_per_token();
    storage.last_update_time = _last_time_reward_applicable();
    storage.rewards.insert(account, _earned(account));
    storage.user_reward_per_token_paid.insert(account, storage.reward_per_token_stored);
}
