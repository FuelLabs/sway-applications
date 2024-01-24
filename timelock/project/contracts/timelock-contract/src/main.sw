contract;

mod data_structures;
mod errors;
mod events;
mod interface;
mod utils;

use ::data_structures::{Asset, ExecutionRange};
use ::errors::{AccessControlError, FundingError, TransactionError};
use ::events::{CancelEvent, ExecuteEvent, QueueEvent};
use ::interface::{Info, Timelock};
use std::{
    auth::msg_sender,
    block::timestamp,
    bytes::Bytes,
    call_frames::msg_asset_id,
    context::this_balance,
    hash::Hash,
};
use ::utils::create_hash;

configurable {
    MAXIMUM_DELAY: u64 = 1000,
    MINIMUM_DELAY: u64 = 100,
    ADMIN: Identity = Identity::Address(Address::from(0x09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db)),
}

storage {
    /// Mapping transaction hash to time range of available execution
    queue: StorageMap<b256, ExecutionRange> = StorageMap {},
}

impl Timelock for Contract {
    #[storage(read, write)]
    fn cancel(id: b256) {
        require(
            msg_sender()
                .unwrap() == ADMIN,
            AccessControlError::AuthorizationError,
        );
        require(
            storage
                .queue
                .get(id)
                .try_read()
                .is_some(),
            TransactionError::InvalidTransaction(id),
        );

        assert(storage.queue.remove(id));

        log(CancelEvent { id })
    }

    #[storage(read, write)]
    fn execute(
        recipient: Identity,
        asset: Option<Asset>,
        data: Option<Bytes>,
        time: u64,
    ) {
        require(
            msg_sender()
                .unwrap() == ADMIN,
            AccessControlError::AuthorizationError,
        );

        let id = create_hash(recipient, asset, data, time);
        let transaction = storage.queue.get(id).try_read();

        require(
            transaction
                .is_some(),
            TransactionError::InvalidTransaction(id),
        );

        // Timestamp is guaranteed to be in the range because of `fn queue()`
        // Therefore, the lower bound can be the timestamp itself; but, we must place an upper bound
        // to prevent going over the MAXIMUM_DELAY
        require(
            time <= timestamp() && timestamp() <= transaction
                .unwrap()
                .end,
            TransactionError::TimestampNotInRange((time, transaction.unwrap().end, timestamp())),
        );

        if asset.is_some() {
            require(
                asset
                    .unwrap()
                    .amount <= this_balance(asset.unwrap().id),
                FundingError::InsufficientContractBalance((this_balance(asset.unwrap().id))),
            );
        }

        assert(storage.queue.remove(id));

        // TODO: execute arbitrary call...
        log(ExecuteEvent {
            asset,
            data,
            id,
            recipient,
            timestamp: time,
        })
    }

    #[storage(read, write)]
    fn queue(
        recipient: Identity,
        asset: Option<Asset>,
        data: Option<Bytes>,
        time: u64,
    ) {
        require(
            msg_sender()
                .unwrap() == ADMIN,
            AccessControlError::AuthorizationError,
        );

        let id = create_hash(recipient, asset, data, time);
        let transaction = storage.queue.get(id).try_read();

        require(
            transaction
                .is_none(),
            TransactionError::DuplicateTransaction(id),
        );

        let start = timestamp() + MINIMUM_DELAY;
        let end = timestamp() + MAXIMUM_DELAY;

        require(
            start <= time && time <= end,
            TransactionError::TimestampNotInRange((start, end, time)),
        );

        storage.queue.insert(id, ExecutionRange { start, end });

        log(QueueEvent {
            asset,
            data,
            id,
            recipient,
            timestamp: time,
        })
    }
}

impl Info for Contract {
    fn balance(asset_id: AssetId) -> u64 {
        this_balance(asset_id)
    }

    fn delays() -> (u64, u64) {
        (MINIMUM_DELAY, MAXIMUM_DELAY)
    }

    #[storage(read)]
    fn queued(id: b256) -> Option<ExecutionRange> {
        storage.queue.get(id).try_read()
    }

    fn transaction_hash(
        recipient: Identity,
        asset: Option<Asset>,
        data: Option<Bytes>,
        timestamp: u64,
    ) -> b256 {
        create_hash(recipient, asset, data, timestamp)
    }
}
