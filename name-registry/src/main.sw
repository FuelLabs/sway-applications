contract;

dep data_structures;
dep errors;
dep events;
dep interface;

use data_structures::Record;
use errors::{AssetErrors, AuthorisationErrors, ValidityErrors};
use events::{
    IdentityChangedEvent,
    NameRegisteredEvent,
    OwnerChangedEvent,
    RegistrationExtendedEvent,
};
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

const ASSET_ID = ~Address::from(ASSET_B256);

// TODO: Change the static 8 length str with a dynamic string when possible
impl NameRegistry for Contract {
    #[storage(read, write)]
    fn extend(name: str[8], duration: u64) {
        require(storage.names.get(name).is_some(), ValidityErrors::NameNotRegistered);
        require((duration / 100) * PRICE_PER_HUNDRED <= msg_amount(), AssetErrors::InsufficientPayment);
        require(msg_asset_id() == BASE_ASSET_ID, AssetErrors::IncorrectAssetSent);

        let previous_record = storage.names.get(name).unwrap();
        let new_record = Record {
            expiry: previous_record.expiry + duration,
            identity: previous_record.identity,
            owner: previous_record.owner,
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

        require((duration / 100) * PRICE_PER_HUNDRED <= msg_amount(), AssetErrors::InsufficientPayment);
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
        let previous_record = storage.names.get(name).unwrap();
        require(timestamp() < previous_record.expiry, ValidityErrors::NameExpired);
        require(previous_record.owner == msg_sender().unwrap(), AuthorisationErrors::SenderNotOwner);

        let new_record = Record {
            expiry: previous_record.expiry,
            identity,
            owner: previous_record.owner,
        };

        storage.names.insert(name, Option::Some(new_record));

        log(IdentityChangedEvent {
            name,
            new_identity: new_record.identity,
            previous_identity: previous_record.identity,
        });
    }

    #[storage(read, write)]
    fn set_owner(name: str[8], new_owner: Identity) {
        require(storage.names.get(name).is_some(), ValidityErrors::NameNotRegistered);
        let previous_record = storage.names.get(name).unwrap();
        require(timestamp() < previous_record.expiry, ValidityErrors::NameExpired);
        require(previous_record.owner == msg_sender().unwrap(), AuthorisationErrors::SenderNotOwner);

        let new_record = Record {
            expiry: previous_record.expiry,
            identity: previous_record.identity,
            owner: new_owner,
        };

        storage.names.insert(name, Option::Some(new_record));

        log(OwnerChangedEvent {
            name,
            new_owner: new_record.owner,
            previous_owner: previous_record.owner,
        });
    }

    #[storage(read)]
    fn expiry(name: str[8]) -> u64 {
        require(storage.names.get(name).is_some(), ValidityErrors::NameNotRegistered);
        let record = storage.names.get(name).unwrap();
        require(timestamp() < record.expiry, ValidityErrors::NameExpired);
        record.expiry
    }

    #[storage(read)]
    fn identity(name: str[8]) -> Identity {
        require(storage.names.get(name).is_some(), ValidityErrors::NameNotRegistered);
        let record = storage.names.get(name).unwrap();
        require(timestamp() < record.expiry, ValidityErrors::NameExpired);
        record.identity
    }

    #[storage(read)]
    fn owner(name: str[8]) -> Identity {
        require(storage.names.get(name).is_some(), ValidityErrors::NameNotRegistered);
        let record = storage.names.get(name).unwrap();
        require(timestamp() < record.expiry, ValidityErrors::NameExpired);
        record.owner
    }
}
