contract;

// use std::inputs::input_amount;

abi CallableContract {
    #[storage(write)]
    fn change_mapping_without_value(identity: Identity, value: u64);

    //payable
    #[storage(write)]
    fn change_mapping_with_value(identity: Identity, value: u64);

    #[storage(read)]
    fn check_counter_map(identity: Identity) -> u64;

    #[storage(read)]
    fn check_deposit_map(identity: Identity) -> u64;
}

storage {
    counter_map: StorageMap<Identity, u64> = StorageMap {},
    deposit_map: StorageMap<Identity, u64> = StorageMap {},
}

impl CallableContract for Contract {
     #[storage(write)]
    fn change_mapping_without_value(identity: Identity, value: u64) {
        storage.counter_map.insert(identity, value);
    }

    //payable
     #[storage(write)]
    fn change_mapping_with_value(identity: Identity, value: u64) {
        storage.counter_map.insert(identity, value);
        // storage.deposit_map.insert(identity, input_amount(0).unwrap_or(0));
    }

    #[storage(read)]
    fn check_counter_map(identity: Identity) -> u64 {
        storage.counter_map.get(identity)
    }

    #[storage(read)]
    fn check_deposit_map(identity: Identity) -> u64 {
        storage.deposit_map.get(identity)
    }
}
