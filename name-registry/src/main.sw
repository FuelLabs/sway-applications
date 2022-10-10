contract;

dep data_structures;
dep errors;
dep events;
dep interface;

use data_structures::Record;
use errors::Errors;
use events::{IdentityChanged, NameRegistered, OwnerChanged, RegistrationExtended};
use interface::NameRegistry;
use std::{
    block::timestamp,
    chain::auth::msg_sender,
    constants::{
        BASE_ASSET_ID,
    },
    context::call_frames::msg_asset_id,
    context::msg_amount,
    logging::log,
    option::Option,
    result::Result,
    storage::StorageMap,
};

storage {
    /// A mapping of names to an option of records, with a none representing an unregistered name
    names: StorageMap<str[8], Option<Record>> = StorageMap {},
}

/// The amount to charge per hundred seconds per name
const PRICE_PER_HUNDRED: u64 = 1;

impl NameRegistry for Contract {
    #[storage(read)]
    fn expiry(name: str[8]) -> u64 {
        match storage.names.get(name) {
            Option::Some(record) => {
                record.expiry
            },
            None => {
                log(Errors::NameNotRegistered);
                revert(0)
            }
        }
    }

    #[storage(read, write)]
    fn extend(name: str[8], duration: u64) {
        require(storage.names.get(name).is_some(), Errors::NameNotRegistered);
        require(duration / 100 * PRICE_PER_HUNDRED <= msg_amount(), Errors::InsufficientPayment);
        require(msg_asset_id() == BASE_ASSET_ID, Errors::WrongAssetSent);

        let old_record = storage.names.get(name).unwrap();
        let new_record = Record {
            expiry: old_record.expiry + duration,
            identity: old_record.identity,
            owner: old_record.owner,
        };

        storage.names.insert(name, Option::Some(new_record));

        log(RegistrationExtended {
            duration,
            name,
            new_expiry: new_record.expiry,
        });
    }

    #[storage(read)]
    fn identity(name: str[8]) -> Identity {
        match storage.names.get(name) {
            Option::Some(record) => {
                record.identity
            },
            None => {
                log(Errors::NameNotRegistered);
                revert(0)
            }
        }
    }

    #[storage(read)]
    fn owner(name: str[8]) -> Identity {
        match storage.names.get(name) {
            Option::Some(record) => {
                record.owner
            },
            None => {
                log(Errors::NameNotRegistered);
                revert(0)
            }
        }
    }

    #[storage(read, write)]
    fn register(name: str[8], duration: u64) {
        if storage.names.get(name).is_some() {
            let record = storage.names.get(name).unwrap();
            require(timestamp() > record.expiry, Errors::NameNotExpired);
        }

        require(duration / 100 * PRICE_PER_HUNDRED <= msg_amount(), Errors::InsufficientPayment);
        require(msg_asset_id() == BASE_ASSET_ID, Errors::WrongAssetSent);

        let record = Record {
            expiry: timestamp() + duration,
            identity: msg_sender().unwrap(),
            owner: msg_sender().unwrap(),
        };

        storage.names.insert(name, Option::Some(record));

        log(NameRegistered {
            expiry: record.expiry,
            name,
            owner: record.owner,
        });
    }
    #[storage(read, write)]
    fn set_identity(name: str[8], identity: Identity) {
        require(storage.names.get(name).is_some(), Errors::NameNotRegistered);
        let old_record = storage.names.get(name).unwrap();
        require(old_record.owner == msg_sender().unwrap(), Errors::SenderNotOwner);

        let new_record = Record {
            expiry: old_record.expiry,
            identity,
            owner: old_record.owner,
        };

        storage.names.insert(name, Option::Some(new_record));

        log(IdentityChanged {
            name,
            new_identity: new_record.identity,
            old_identity: old_record.identity,
        });
    }

    #[storage(read, write)]
    fn set_owner(name: str[8], new_owner: Identity) {
        require(storage.names.get(name).is_some(), Errors::NameNotRegistered);
        let old_record = storage.names.get(name).unwrap();
        require(old_record.owner == msg_sender().unwrap(), Errors::SenderNotOwner);

        let new_record = Record {
            expiry: old_record.expiry,
            identity: old_record.identity,
            owner: new_owner,
        };

        storage.names.insert(name, Option::Some(new_record));

        log(OwnerChanged {
            name,
            new_owner: new_record.owner,
            old_owner: old_record.owner,
        });
    }
}
