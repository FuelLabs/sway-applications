library staking_rewards_abi;

use std::contract_id::ContractId;
use std::identity::Identity;

abi StakingRewards {
    #[storage(read)]fn balance_of(account: Identity) -> u64;
    #[storage(read)]fn earned(account: Identity, test_timestamp: u64) -> u64;
    #[storage(read)]fn get_reward_for_duration() -> u64;
    #[storage(read)]fn last_time_reward_applicable(test_timestamp: u64) -> u64;
    #[storage(read)]fn reward_per_token(test_timestamp: u64) -> u64;
    #[storage(read)]fn rewards_distribution() -> Identity;
    #[storage(read)]fn rewards_token() -> ContractId;
    #[storage(read)]fn total_supply() -> u64;

    #[storage(read, write)]fn exit(test_timestamp: u64);
    #[storage(read, write)]fn get_reward(test_timestamp: u64);
    #[storage(read, write)]fn stake(test_timestamp: u64);
    #[storage(read, write)]fn withdraw(amount: u64, test_timestamp: u64);
}
