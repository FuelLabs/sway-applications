library staking_rewards_abi;

use std::address::Address;
use std::contract_id::ContractId;

abi StakingRewards {
    #[storage(read)]fn balance_of(account: Address) -> u64;
    #[storage(read)]fn earned(account: Address) -> u64;
    #[storage(read)]fn get_reward_for_duration() -> u64;
    #[storage(read)]fn last_time_reward_applicable() -> u64;
    #[storage(read)]fn reward_per_token() -> u64;
    #[storage(read)]fn rewards_distribution() -> Address;
    #[storage(read)]fn rewards_token() -> ContractId;
    #[storage(read)]fn total_supply() -> u64;

    #[storage(read, write)]fn exit();
    #[storage(read, write)]fn get_reward();
    #[storage(read, write)]fn stake(amount: u64);
    #[storage(read, write)]fn withdraw(amount: u64);
}
