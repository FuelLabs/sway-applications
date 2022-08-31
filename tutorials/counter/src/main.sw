contract;

dep interface;

use interface::Counter;

storage {
    counter: u64 = 0
}

impl Counter for Contract {
    #[storage(read,write)]fn increment() {
        storage.counter = storage.counter + 10;
    }

    #[storage(read,write)]fn decrement() {
        if storage.counter != 0 {
            storage.counter = storage.counter - 10;
        }
    }

    #[storage(read)]fn get_counter() -> u64 {
        storage.counter
    }
}
