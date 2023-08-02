library;

abi Counter {
    #[storage(read, write)]
    fn increment() -> u64;
    #[storage(read)]
    fn count() -> u64;
    #[storage(write)]
    fn clear();
}
