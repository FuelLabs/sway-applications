contract;

mod data_structures;
mod errors;
mod events;
mod interface;

use ::data_structures::Record;
use ::errors::{AssetError, AuthorisationError, RegistrationValidityError};
use ::events::{
    IdentityChangedEvent,
    NameRegisteredEvent,
    OwnerChangedEvent,
    RegistrationExtendedEvent,
};
use ::interface::{Info, NameRegistry};
use std::{block::timestamp, call_frames::msg_asset_id, context::msg_amount, string::String};

// Amount of units of the asset to charge per 100 seconds of registration duration for every name/entry
configurable {
    ASSET_ID: ContractId = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
    PRICE_PER_HUNDRED: u64 = 1,
}

storage {
    /// A mapping of names to an option of records, with a none representing an unregistered name
    names: StorageMap<b256, Record> = StorageMap {},
}

impl NameRegistry for Contract {
    #[payable]
    #[storage(read, write)]
    fn extend(name: String, duration: u64) {
        let entry = name.bytes.sha256();
        require(storage.names.get(entry).try_read().is_some(), RegistrationValidityError::NameNotRegistered);
        require(msg_asset_id() == ASSET_ID, AssetError::IncorrectAssetSent);
        require((duration / 100) * PRICE_PER_HUNDRED <= msg_amount(), AssetError::InsufficientPayment);

        let mut record = storage.names.get(entry).try_read().unwrap();
        record.expiry = record.expiry + duration;

        storage.names.insert(entry, record);

        log(RegistrationExtendedEvent {
            duration,
            name,
            new_expiry: record.expiry,
        });
    }

    #[payable]
    #[storage(read, write)]
    fn register(
        name: String,
        duration: u64,
        owner: Identity,
        identity: Identity,
    ) {
        let entry = name.bytes.sha256();
        if storage.names.get(entry).try_read().is_some() {
            let record = storage.names.get(entry).try_read().unwrap();
            require(timestamp() > record.expiry, RegistrationValidityError::NameNotExpired);
        }

        require(msg_asset_id() == ASSET_ID, AssetError::IncorrectAssetSent);
        require((duration / 100) * PRICE_PER_HUNDRED <= msg_amount(), AssetError::InsufficientPayment);

        let record = Record::new(timestamp() + duration, identity, owner);

        storage.names.insert(entry, record);

        log(NameRegisteredEvent {
            expiry: record.expiry,
            name,
            owner,
            identity,
        });
    }

    #[storage(read, write)]
    fn set_identity(name: String, identity: Identity) {
        let entry = name.bytes.sha256();
        require(storage.names.get(entry).try_read().is_some(), RegistrationValidityError::NameNotRegistered);
        let previous_record = storage.names.get(entry).try_read().unwrap();
        require(timestamp() < previous_record.expiry, RegistrationValidityError::NameExpired);
        require(previous_record.owner == msg_sender().unwrap(), AuthorisationError::SenderNotOwner);

        let new_record = Record::new(previous_record.expiry, identity, previous_record.owner);

        storage.names.insert(entry, new_record);

        log(IdentityChangedEvent {
            name,
            new_identity: new_record.identity,
            previous_identity: previous_record.identity,
        });
    }

    #[storage(read, write)]
    fn set_owner(name: String, owner: Identity) {
        let entry = name.bytes.sha256();
        require(storage.names.get(entry).try_read().is_some(), RegistrationValidityError::NameNotRegistered);
        let previous_record = storage.names.get(entry).try_read().unwrap();
        require(timestamp() < previous_record.expiry, RegistrationValidityError::NameExpired);
        require(previous_record.owner == msg_sender().unwrap(), AuthorisationError::SenderNotOwner);

        let new_record = Record::new(previous_record.expiry, previous_record.identity, owner);

        storage.names.insert(entry, new_record);

        log(OwnerChangedEvent {
            name,
            new_owner: new_record.owner,
            previous_owner: previous_record.owner,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn expiry(name: String) -> Result<u64, RegistrationValidityError> {
        match storage.names.get(name.bytes.sha256()).try_read() {
            Option::Some(record) => {
                match timestamp() < record.expiry {
                    true => Result::Ok(record.expiry),
                    false => Result::Err(RegistrationValidityError::NameExpired),
                }
            },
            Option::None => Result::Err(RegistrationValidityError::NameNotRegistered),
        }
    }

    #[storage(read)]
    fn identity(name: String) -> Result<Identity, RegistrationValidityError> {
        match storage.names.get(name.bytes.sha256()).try_read() {
            Option::Some(record) => {
                match timestamp() < record.expiry {
                    true => Result::Ok(record.identity),
                    false => Result::Err(RegistrationValidityError::NameExpired),
                }
            },
            Option::None => Result::Err(RegistrationValidityError::NameNotRegistered),
        }
    }

    #[storage(read)]
    fn owner(name: String) -> Result<Identity, RegistrationValidityError> {
        match storage.names.get(name.bytes.sha256()).try_read() {
            Option::Some(record) => {
                match timestamp() < record.expiry {
                    true => Result::Ok(record.owner),
                    false => Result::Err(RegistrationValidityError::NameExpired),
                }
            },
            Option::None => Result::Err(RegistrationValidityError::NameNotRegistered),
        }
    }
}
