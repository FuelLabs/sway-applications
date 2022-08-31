library interface;

abi Counter {
    #[storage(read,write)]fn increment();
    #[storage(read,write)]fn decrement();
    #[storage(read)]fn get_counter() -> u64;
}
