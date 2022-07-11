contract;

dep staking_rewards_abi;

use std::{
    address::Address,
    constants::ZERO_B256,
    contract_id::ContractId,
    identity::Identity,
    storage::StorageMap,
};

use staking_rewards_abi::StakingRewards;

storage {
    rewards_token: ContractId = ContractId {
        value: 0x0000000000000000000000000000000000000000000000000000000000000000,
    },
    staking_token: ContractId = ContractId {
        value: 0x0000000000000000000000000000000000000000000000000000000000000000,
    },
    period_finish: u64 = 0,
    reward_rate: u64 = 0,
    rewards_duration: u64 = 0,
    last_update_time: u64 = 1,
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
}

impl StakingRewards for Contract {
    #[storage(read)]fn balance_of(account: Identity) -> u64 {
        storage.balances.get(account)
    }

    #[storage(read)]fn earned(account: Identity) -> u64 {
        0
    }

    #[storage(read)]fn get_reward_for_duration() -> u64 {
        0
    }

    #[storage(read)]fn last_time_reward_applicable() -> u64 {
        // TODO (functionality): use block timestamp once implemented
        let timestamp = 0;
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

    #[storage(read)]fn reward_per_token() -> u64 {
        0
    }

    #[storage(read)]fn rewards_distribution() -> Identity {
        Identity::ContractId(ContractId {
            value: ZERO_B256
        })
    }

    #[storage(read)]fn rewards_token() -> ContractId {
        ContractId {
            value: ZERO_B256,
        }
    }

    #[storage(read)]fn total_supply() -> u64 {
        storage.total_supply
    }

    #[storage(read, write)]fn exit() {
    }

    #[storage(read, write)]fn get_reward() {
    }

    #[storage(read, write)]fn stake(amount: u64) {
    }

    #[storage(read, write)]fn withdraw(amount: u64) {
    }
}
