contract;

mod data_structures;
mod errors;
mod events;
mod interface;

use ::data_structures::Record;
use ::errors::{AssetError, AuthorizationError, RegistrationValidityError};
use ::events::{
    AssetRateEvent,
    IdentityChangedEvent,
    NameRegisteredEvent,
    OwnerChangedEvent,
    RegistrationExtendedEvent,
};
use ::interface::{Info, NameRegistry};
use std::{
    block::timestamp,
    call_frames::msg_asset_id,
    constants::ZERO_B256,
    context::msg_amount,
    hash::{
        Hash,
        sha256,
    },
    storage::storage_string::*,
    string::String,
};

configurable {
    /// The privileged user with the ability to set assets for payment for the registry
    OWNER: Identity = Identity::Address(Address::from(ZERO_B256)),
}

storage {
    /// Cost rate per asset
    assets: StorageMap<AssetId, Option<u64>> = StorageMap {},
    /// A mapping of names to records
    names: StorageMap<b256, Record> = StorageMap {},
}

impl NameRegistry for Contract {
    #[payable]
    #[storage(read, write)]
    fn extend(name: String, duration: u64) {
        // Get record
        let name_hash = sha256(name);
        let record = storage.names.get(name_hash).try_read();
        require(
            record
                .is_some(),
            RegistrationValidityError::NameNotRegistered,
        );

        // Verify payment
        let payment_asset = msg_asset_id();
        let rate = storage.assets.get(payment_asset).try_read();
        require(rate.unwrap().is_some(), AssetError::IncorrectAssetSent);
        require(
            (duration / 100) * rate
                .unwrap()
                .unwrap() <= msg_amount(),
            AssetError::InsufficientPayment,
        );

        // Update stored record
        let mut record = record.unwrap();
        record.expiry = record.expiry + duration;
        storage.names.insert(name_hash, record);

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
        require(
            name
                .as_bytes()
                .len() >= 3,
            RegistrationValidityError::NameTooShort,
        );

        // Get record
        let name_hash = sha256(name);
        let record = storage.names.get(name_hash).try_read();
        if record.is_some() {
            require(
                timestamp() > record
                    .unwrap()
                    .expiry,
                RegistrationValidityError::NameNotExpired,
            );
        }

        // Verify payment
        let payment_asset = msg_asset_id();
        let rate = storage.assets.get(payment_asset).try_read();
        require(rate.unwrap().is_some(), AssetError::IncorrectAssetSent);
        require(
            (duration / 100) * rate
                .unwrap()
                .unwrap() <= msg_amount(),
            AssetError::InsufficientPayment,
        );

        // Store record
        let record = Record::new(timestamp() + duration, identity, owner);
        storage.names.insert(name_hash, record);

        log(NameRegisteredEvent {
            expiry: record.expiry,
            name,
            owner,
            identity,
        });
    }

    #[storage(write)]
    fn set_asset(asset: AssetId, rate: Option<u64>) {
        require(
            msg_sender()
                .unwrap() == OWNER,
            AuthorizationError::SenderNotOwner,
        );
        storage.assets.insert(asset, rate);
        log(AssetRateEvent { asset, rate });
    }

    #[storage(read, write)]
    fn set_resolver(name: String, identity: Identity) {
        // Get record
        let name_hash = sha256(name);
        let record = storage.names.get(name_hash).try_read();
        require(
            record
                .is_some(),
            RegistrationValidityError::NameNotRegistered,
        );

        // Verify record
        let previous_record = record.unwrap();
        require(
            timestamp() < previous_record
                .expiry,
            RegistrationValidityError::NameExpired,
        );
        require(
            previous_record
                .owner == msg_sender()
                .unwrap(),
            AuthorizationError::SenderNotOwner,
        );

        // Store updated record
        let new_record = Record::new(previous_record.expiry, identity, previous_record.owner);
        storage.names.insert(name_hash, new_record);

        log(IdentityChangedEvent {
            name,
            new_identity: new_record.identity,
            previous_identity: previous_record.identity,
        });
    }

    #[storage(read, write)]
    fn transfer_name_ownership(name: String, owner: Identity) {
        // Get record
        let name_hash = sha256(name);
        let record = storage.names.get(name_hash).try_read();
        require(
            record
                .is_some(),
            RegistrationValidityError::NameNotRegistered,
        );

        // Verify record
        let previous_record = record.unwrap();
        require(
            timestamp() < previous_record
                .expiry,
            RegistrationValidityError::NameExpired,
        );
        require(
            previous_record
                .owner == msg_sender()
                .unwrap(),
            AuthorizationError::SenderNotOwner,
        );

        // Store updated record
        let new_record = Record::new(previous_record.expiry, previous_record.identity, owner);
        storage.names.insert(name_hash, new_record);

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
        let name_hash = sha256(name);
        match storage.names.get(name_hash).try_read() {
            Some(record) => {
                match timestamp() < record.expiry {
                    true => Ok(record.expiry),
                    false => Err(RegistrationValidityError::NameExpired),
                }
            },
            None => Err(RegistrationValidityError::NameNotRegistered),
        }
    }

    #[storage(read)]
    fn resolver(name: String) -> Result<Identity, RegistrationValidityError> {
        let name_hash = sha256(name);
        match storage.names.get(name_hash).try_read() {
            Some(record) => {
                match timestamp() < record.expiry {
                    true => Ok(record.identity),
                    false => Err(RegistrationValidityError::NameExpired),
                }
            },
            None => Err(RegistrationValidityError::NameNotRegistered),
        }
    }

    #[storage(read)]
    fn name_owner(name: String) -> Result<Identity, RegistrationValidityError> {
        let name_hash = sha256(name);
        match storage.names.get(name_hash).try_read() {
            Some(record) => {
                match timestamp() < record.expiry {
                    true => Ok(record.owner),
                    false => Err(RegistrationValidityError::NameExpired),
                }
            },
            None => Err(RegistrationValidityError::NameNotRegistered),
        }
    }

    #[storage(read)]
    fn rate(asset: AssetId) -> Option<u64> {
        match storage.assets.get(asset).try_read() {
            Some(rate) => rate,
            None => None,
        }
    }
}
