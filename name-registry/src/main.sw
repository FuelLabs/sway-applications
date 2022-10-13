contract;

dep data_structures;
dep errors;
dep events;
dep interface;

use data_structures::Record;
use errors::{AssetErrors, AuthorisationErrors, ValidityErrors};
use events::{IdentityChanged, NameRegistered, OwnerChanged, RegistrationExtended};
use interface::NameRegistry;
use std::{
    block::timestamp,
    chain::auth::msg_sender,
    constants::BASE_ASSET_ID,
    context::call_frames::msg_asset_id,
    context::msg_amount,
    logging::log,
    storage::StorageMap,
};

storage {
    /// A mapping of names to an option of records, with a none representing an unregistered name
    names: StorageMap<str[8], Option<Record>> = StorageMap {},
}

// TODO: Change the static 8 length str with a dynamic string when possible
impl NameRegistry for Contract {
    #[storage(read, write)]
    fn extend(name: str[8], duration: u64) {
        require(storage.names.get(name).is_some(), ValidityErrors::NameNotRegistered);
        require((duration / 100) * price_per_hundred <= msg_amount(), AssetErrors::InsufficientPayment);
        require(msg_asset_id() == BASE_ASSET_ID, AssetErrors::IncorrectAssetSent);

        let old_record = storage.names.get(name).unwrap();
        let new_record = Record {
            expiry: old_record.expiry + duration,
            identity: old_record.identity,
            owner: old_record.owner,
        };

        storage.names.insert(name, Option::Some(new_record));

        log(RegistrationExtendedEvent {
            duration,
            name,
            new_expiry: new_record.expiry,
        });
    }

    #[storage(read, write)]
    fn register(
        name: str[8],
        duration: u64,
        owner: Identity,
        identity: Identity,
    ) {
        if storage.names.get(name).is_some() {
            let record = storage.names.get(name).unwrap();
            require(timestamp() > record.expiry, ValidityErrors::NameNotExpired);
        }

        require((duration / 100) * price_per_hundred <= msg_amount(), AssetErrors::InsufficientPayment);
        require(msg_asset_id() == BASE_ASSET_ID, AssetErrors::IncorrectAssetSent);

        let record = Record {
            expiry: timestamp() + duration,
            identity,
            owner,
        };

        storage.names.insert(name, Option::Some(record));

        log(NameRegisteredEvent {
            expiry: record.expiry,
            name,
            owner,
            identity,
        });
    }

    #[storage(read, write)]
    fn set_identity(name: str[8], identity: Identity) {
        require(storage.names.get(name).is_some(), ValidityErrors::NameNotRegistered);
        let old_record = storage.names.get(name).unwrap();
        require(timestamp() < old_record.expiry, ValidityErrors::NameExpired);
        require(old_record.owner == msg_sender().unwrap(), AuthorisationErrors::SenderNotOwner);

        let new_record = Record {
            expiry: old_record.expiry,
            identity,
            owner: old_record.owner,
        };

        storage.names.insert(name, Option::Some(new_record));

        log(IdentityChangedEvent {
            name,
            new_identity: new_record.identity,
            old_identity: old_record.identity,
        });
    }

    #[storage(read, write)]
    fn set_owner(name: str[8], new_owner: Identity) {
        require(storage.names.get(name).is_some(), ValidityErrors::NameNotRegistered);
        let old_record = storage.names.get(name).unwrap();
        require(timestamp() < old_record.expiry, ValidityErrors::NameExpired);
        require(old_record.owner == msg_sender().unwrap(), AuthorisationErrors::SenderNotOwner);

        let new_record = Record {
            expiry: old_record.expiry,
            identity: old_record.identity,
            owner: new_owner,
        };

        storage.names.insert(name, Option::Some(new_record));

        log(OwnerChangedEvent {
            name,
            new_owner: new_record.owner,
            old_owner: old_record.owner,
        });
    }

    #[storage(read)]
    fn expiry(name: str[8]) -> u64 {
        match storage.names.get(name) {
            Option::Some(record) => {
                require(timestamp() < record.expiry, ValidityErrors::NameExpired);
                record.expiry
            },
            None => {
                log(ValidityErrors::NameNotRegistered);
                revert(0)
            }
        }
    }

    #[storage(read)]
    fn identity(name: str[8]) -> Identity {
        match storage.names.get(name) {
            Option::Some(record) => {
                require(timestamp() < record.expiry, ValidityErrors::NameExpired);
                record.identity
            },
            None => {
                log(ValidityErrors::NameNotRegistered);
                revert(0)
            }
        }
    }

    #[storage(read)]
    fn owner(name: str[8]) -> Identity {
        match storage.names.get(name) {
            Option::Some(record) => {
                require(timestamp() < record.expiry, ValidityErrors::NameExpired);
                record.owner
            },
            None => {
                log(ValidityErrors::NameNotRegistered);
                revert(0)
            }
        }
    }
}
