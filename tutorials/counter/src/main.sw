contract;

use std::*;
use core::*;
use std::storage::*;

abi CounterContract {
    fn increment_counter(value: u64) -> u64;
    fn decrement_counter(value: u64) -> u64;
    fn get_counter() -> u64;
}

const COUNTER_KEY = 0x0000000000000000000000000000000000000000000000000000000000000000;

impl CounterContract for Contract {
    fn increment_counter(value: u64) -> u64 {
        let new_value = get::<u64>(COUNTER_KEY) + value;
        store(COUNTER_KEY, new_value);
        new_value
    }
    fn decrement_counter(value: u64) -> u64 {
        let new_value = get::<u64>(COUNTER_KEY) - value;
        store(COUNTER_KEY, new_value);
        new_value
    }
    fn get_counter() -> u64 {
        get::<u64>(COUNTER_KEY)
    }
}
