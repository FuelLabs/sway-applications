contract;

dep data_structures;
dep errors;
dep interface;

use data_structures::Record;
use errors::Errors;
use interface::NameRegistry;
use std::{
    revert::require,
    block::timestamp,
    context::msg_amount,
    context::call_frames::msg_asset_id,
    constants::{
        BASE_ASSET_ID,
        ZERO_B256,
    },
    chain::auth::msg_sender,
    storage::StorageMap
};

storage {
    names: StorageMap<str[8], Option<Record>> = StorageMap {},
}

const PRICE_PER_HUNDRED: u64 = 1;

impl NameRegistry for Contract {
    #[storage(read, write)]
    fn extend(name: str[8], duration: u64) {
        require(storage.names.get(name).is_some(), Errors::NameNotRegistered);
        require(duration/100 * PRICE_PER_HUNDRED <= msg_amount(), Errors::InsufficientPayment);
        require(msg_asset_id() == BASE_ASSET_ID, Errors::WrongAssetSent);

        let record = storage.names.get(name).unwrap();

        storage.names.insert(name, Option::Some(Record {
            expiry: record.expiry + duration,
            identity: record.identity,
            owner: record.owner,
        }))
    }

    #[storage(read, write)]
    fn register(name: str[8], duration: u64) {
        if storage.names.get(name).is_some() { 
            let record = storage.names.get(name).unwrap();
            require(timestamp() > record.expiry, Errors::NameNotExpired);
        }

        require(duration/100 * PRICE_PER_HUNDRED <= msg_amount(), Errors::InsufficientPayment);
        require(msg_asset_id() == BASE_ASSET_ID, Errors::WrongAssetSent);

        storage.names.insert(name, Option::Some(Record {
            expiry: timestamp() + duration,
            identity: msg_sender().unwrap(),
            owner: msg_sender().unwrap(),
        }));
    }    

    #[storage(read, write)]
    fn set_identity(name: str[8], identity: Identity) {
        require(storage.names.get(name).is_some(), Errors::NameNotRegistered);
        let record = storage.names.get(name).unwrap();
        require(record.owner == msg_sender().unwrap(), Errors::SenderNotOwner);

        storage.names.insert(name, Option::Some(Record {
            expiry: record.expiry,
            identity,
            owner: record.owner,
        }))
    }

    #[storage(read, write)]
    fn set_owner(name: str[8], new_owner: Identity) {
        require(storage.names.get(name).is_some(), Errors::NameNotRegistered);
        let record = storage.names.get(name).unwrap();
        require(record.owner == msg_sender().unwrap(), Errors::SenderNotOwner);

        storage.names.insert(name, Option::Some(Record {
            expiry: record.expiry,
            identity: record.identity,
            owner: new_owner,
        }))
    }
}
