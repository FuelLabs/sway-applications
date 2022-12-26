library interface;

use std::bytes::Bytes;

abi Timelock {
    #[storage(read, write)]
    fn cancel(id: b256);

    #[storage(read, write)]
    fn execute(recipient: Identity, value: u64, data: Bytes, timestamp: u64);

    #[storage(read, write)]
    fn queue(recipient: Identity, value: u64, data: Bytes, timestamp: u64);
}

abi Info {
    #[storage(read)]
    fn queued(id: b256) -> bool;

    fn transaction_hash(recipient: Identity, value: u64, data: Bytes, timestamp: u64) -> b256;
}
