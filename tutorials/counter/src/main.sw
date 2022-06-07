contract;

dep abi;

use abi::Counter;

storage {
    counter: u64
}

impl Counter for Contract {
    fn increment() {
        storage.counter = storage.counter + 1;
    }

    fn decrement() {
        if storage.counter != 0 {
            storage.counter = storage.counter - 1;
        }
    }

    fn get_counter() -> u64 {
        storage.counter
    }
}
