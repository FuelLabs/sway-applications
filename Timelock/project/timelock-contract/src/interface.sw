library interface;

dep data_structures;

use data_structures::ExecutionRange;
use std::bytes::Bytes;

abi Timelock {
    ///
    ///
    /// # Arguments
    ///
    /// - `id`:
    ///
    /// # Reverts
    ///
    ///
    #[storage(read, write)]
    fn cancel(id: b256);

    ///
    ///
    /// # Arguments
    ///
    /// - `recipient`:
    /// - `value`:
    /// - `data`:
    /// - `timestamp`:
    ///
    /// # Reverts
    ///
    ///
    #[storage(read, write)]
    fn execute(recipient: Identity, value: u64, data: Bytes, timestamp: u64);

    ///
    ///
    /// # Arguments
    ///
    /// - `recipient`:
    /// - `value`:
    /// - `data`:
    /// - `timestamp`:
    ///
    /// # Reverts
    ///
    ///
    #[storage(read, write)]
    fn queue(recipient: Identity, value: u64, data: Bytes, timestamp: u64);
}

abi Info {
    ///
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[storage(read)]
    fn queued(id: b256) -> Option<ExecutionRange>;

    ///
    ///
    /// # Arguments
    ///
    /// - `recipient`:
    /// - `value`:
    /// - `data`:
    /// - `timestamp`:
    fn transaction_hash(recipient: Identity, value: u64, data: Bytes, timestamp: u64) -> b256;
}
