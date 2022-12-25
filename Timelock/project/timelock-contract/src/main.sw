contract;

dep errors;
dep events;
dep interface;
dep utils;

use errors::Error;
use events::{CancelEvent, ExecuteEvent, QueueEvent};
use interface::{Info, Timelock};
use std::{auth::msg_sender, bytes::Bytes, logging::log};
use utils::create_hash;

const ADMIN: Identity = Identity::Address(Address::from(OWNER));

storage {
    queue: StorageMap<b256, bool> = StorageMap {},
}

impl Timelock for Contract {
    #[storage(read, write)]
    fn cancel(id: b256) {
        require(msg_sender().unwrap() == ADMIN, Error::AuthorizationError);
        require(storage.queue.get(id), Error::TransactionCancelled);

        storage.queue.insert(id, false);

        log(CancelEvent { id })
    }

    #[storage(read, write)]
    fn execute() {
        require(msg_sender().unwrap() == ADMIN, Error::AuthorizationError); 

        // TODO: execute arbitrary call...
        log(ExecuteEvent {})
    }

    #[storage(read, write)]
    fn queue(recipient: Identity, value: u64, data: Bytes, timestamp: u64) {
        require(msg_sender().unwrap() == ADMIN, Error::AuthorizationError);

        let id = create_hash(recipient, value, data, timestamp);

        require(!storage.queue.get(id), Error::DuplicateTransaction);


        // TODO: check timestamp is valid
        // require(timestamp_is_valid, Error::InvalidTimestamp);
        storage.queue.insert(id, true);

        log(QueueEvent {
            recipient,
            value,
            data,
            timestamp,
        })
    }
}

impl Info for Contract {
    fn transaction_hash(recipient: Identity, value: u64, data: Bytes, timestamp: u64) -> b256 {
        create_hash(recipient, value, data, timestamp)
    }
}
