contract;

dep errors;
dep events;
dep interface;
dep utils;

use errors::{AccessControlError, TransactionError};
use events::{CancelEvent, ExecuteEvent, QueueEvent};
use interface::{Info, Timelock};
use std::{auth::msg_sender, block::timestamp as now, bytes::Bytes, logging::log};
use utils::create_hash;

const ADMIN: Identity = Identity::Address(Address::from(OWNER));

storage {
    // TODO: change bool to struct containing bool and upper/lower time limit so that we can compare
    //       timerange, make it an option too
    queue: StorageMap<b256, bool> = StorageMap {},
}

impl Timelock for Contract {
    #[storage(read, write)]
    fn cancel(id: b256) {
        require(msg_sender().unwrap() == ADMIN, AccessControlError::AuthorizationError);
        require(storage.queue.get(id), TransactionError::InvalidTransaction(id));

        storage.queue.insert(id, false);

        log(CancelEvent { id })
    }

    #[storage(read, write)]
    fn execute(recipient: Identity, value: u64, data: Bytes, timestamp: u64) {
        require(msg_sender().unwrap() == ADMIN, AccessControlError::AuthorizationError);

        let id = create_hash(recipient, value, data, timestamp);

        require(storage.queue.get(id), TransactionError::InvalidTransaction(id));

        // TODO: make sure timestamp is in range
        storage.queue.insert(id, false);

        // TODO: execute arbitrary call...
        log(ExecuteEvent {
            data,
            id,
            recipient,
            timestamp,
            value,
        })
    }

    #[storage(read, write)]
    fn queue(recipient: Identity, value: u64, data: Bytes, timestamp: u64) {
        require(msg_sender().unwrap() == ADMIN, AccessControlError::AuthorizationError);

        let id = create_hash(recipient, value, data, timestamp);

        require(!storage.queue.get(id), TransactionError::DuplicateTransaction(id));
        require(now() + MINIMUM_DELAY <= timestamp && timestamp <= now() + MAXIMUM_DELAY, TransactionError::TimestampNotInRange((now() + MINIMUM_DELAY, now() + MAXIMUM_DELAY, timestamp)));

        storage.queue.insert(id, true);

        log(QueueEvent {
            data,
            id,
            recipient,
            timestamp,
            value,
        })
    }
}

impl Info for Contract {
    #[storage(read)]
    fn queued(id: b256) -> bool {
        storage.queue.get(id)
    }

    fn transaction_hash(recipient: Identity, value: u64, data: Bytes, timestamp: u64) -> b256 {
        create_hash(recipient, value, data, timestamp)
    }
}
