library interface;

abi Counter {
    fn increment();
    fn decrement();
    fn get_counter() -> u64;
}
