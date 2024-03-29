contract;

use libraries::Counter;

storage {
    /// Internal counter updated via calls from a script.
    count: u64 = 0,
}

impl Counter for Contract {
    #[storage(read, write)]
    fn increment() -> u64 {
        let count = storage.count.read() + 1;
        storage.count.write(count);
        count
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
