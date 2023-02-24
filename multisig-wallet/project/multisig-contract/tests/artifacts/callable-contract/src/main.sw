contract;

// use std::inputs::input_amount;
abi CallableContract {
    #[storage(write)]
    fn change_mapping_without_value(address: Address, value: u64);

    //payable
    #[storage(write)]
    fn change_mapping_with_value(address: Address, value: u64);

    #[storage(read)]
    fn check_counter_map(address: Address) -> u64;

    #[storage(read)]
    fn check_deposit_map(address: Address) -> u64;
}

storage {
    counter_map: StorageMap<Address, u64> = StorageMap {},
    deposit_map: StorageMap<Address, u64> = StorageMap {},
}

impl CallableContract for Contract {
    #[storage(write)]
    fn change_mapping_without_value(address: Address, value: u64) {
        storage.counter_map.insert(address, value);
    }

    //payable
    #[storage(write)]
    fn change_mapping_with_value(address: Address, value: u64) {
        storage.counter_map.insert(address, value);
        // storage.deposit_map.insert(address, input_amount(0).unwrap_or(0));
    }

    #[storage(read)]
    fn check_counter_map(address: Address) -> u64 {
        storage.counter_map.get(address)
    }

    #[storage(read)]
    fn check_deposit_map(address: Address) -> u64 {
        storage.deposit_map.get(address)
    }
}
