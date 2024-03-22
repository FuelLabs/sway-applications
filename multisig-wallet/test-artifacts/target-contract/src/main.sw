contract;

use std::{context::msg_amount, hash::Hash};

abi TargetContract {
    #[storage(write)]
    fn update_counter(address: Address, value: u64);

    #[payable]
    #[storage(write)]
    fn update_deposit(address: Address, value: u64);

    #[storage(read)]
    fn count(address: Address) -> u64;

    #[storage(read)]
    fn deposit(address: Address) -> u64;
}

storage {
    counter: StorageMap<Address, u64> = StorageMap {},
    deposit: StorageMap<Address, u64> = StorageMap {},
}

impl TargetContract for Contract {
    #[storage(write)]
    fn update_counter(address: Address, value: u64) {
        storage.counter.insert(address, value);
    }

    #[payable]
    #[storage(write)]
    fn update_deposit(address: Address, value: u64) {
        storage.counter.insert(address, value);
        storage.deposit.insert(address, msg_amount());
    }

    #[storage(read)]
    fn count(address: Address) -> u64 {
        storage.counter.get(address).try_read().unwrap_or(0)
    }

    #[storage(read)]
    fn deposit(address: Address) -> u64 {
        storage.deposit.get(address).try_read().unwrap_or(0)
    }
}
