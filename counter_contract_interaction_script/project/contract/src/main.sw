contract;

storage {
    count: u64 = 0,
}

abi CounterContract {
    #[storage(read, write)]
    fn increment() -> u64;
    #[storage(read)]
    fn count() -> u64;
    #[storage(write)]
    fn clear();
}

impl CounterContract for Contract {
    #[storage(read, write)]
    fn increment() -> u64 {
        storage.count.write(storage.count.read() + 1);
        storage.count.read()
    }

    #[storage(read)]
    fn count() -> u64 {
        storage.count.read()
    }

    #[storage(write)]
    fn clear() {
        storage.count.write(0);
    }
}
