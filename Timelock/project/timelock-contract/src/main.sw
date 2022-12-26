contract;

dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use data_structures::ExecutionRange;
use errors::{AccessControlError, TransactionError};
use events::{CancelEvent, ExecuteEvent, QueueEvent};
use interface::{Info, Timelock};
use std::{auth::msg_sender, block::timestamp as now, bytes::Bytes, logging::log};
use utils::create_hash;

// TODO: reconsider the semantics of using "timestamp" and the logic
const ADMIN: Identity = Identity::Address(Address::from(OWNER));

storage {
    /// Mapping transaction hash to time range of available execution
    queue: StorageMap<b256, Option<ExecutionRange>> = StorageMap {},
}

impl Timelock for Contract {
    #[storage(read, write)]
    fn cancel(id: b256) {
        require(msg_sender().unwrap() == ADMIN, AccessControlError::AuthorizationError);
        require(storage.queue.get(id).is_some(), TransactionError::InvalidTransaction(id));

        storage.queue.insert(id, Option::None::<ExecutionRange>());

        log(CancelEvent { id })
    }

    #[storage(read, write)]
    fn execute(recipient: Identity, value: u64, data: Bytes, timestamp: u64) {
        require(msg_sender().unwrap() == ADMIN, AccessControlError::AuthorizationError);

        let id = create_hash(recipient, value, data, timestamp);
        let transaction = storage.queue.get(id);

        require(transaction.is_some(), TransactionError::InvalidTransaction(id));
        require(transaction.unwrap().start <= now() && now() <= transaction.unwrap().end, TransactionError::TimestampNotInRange((
            transaction.unwrap().start,
            transaction.unwrap().end,
            now(),
        )));

        storage.queue.insert(id, Option::None::<ExecutionRange>());

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
        let transaction = storage.queue.get(id);

        require(transaction.is_none(), TransactionError::DuplicateTransaction(id));
        require(now() + MINIMUM_DELAY <= timestamp && timestamp <= now() + MAXIMUM_DELAY, TransactionError::TimestampNotInRange((now() + MINIMUM_DELAY, now() + MAXIMUM_DELAY, timestamp)));

        storage.queue.insert(id, Option::Some(ExecutionRange {
            start: now() + timestamp + MINIMUM_DELAY,
            end: now() + timestamp + MAXIMUM_DELAY,
        }));

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
    fn queued(id: b256) -> Option<ExecutionRange> {
        storage.queue.get(id)
    }

    fn transaction_hash(recipient: Identity, value: u64, data: Bytes, timestamp: u64) -> b256 {
        create_hash(recipient, value, data, timestamp)
    }
}
