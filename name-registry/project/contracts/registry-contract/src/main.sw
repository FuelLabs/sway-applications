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
    auth::msg_sender,
    block::timestamp,
    call_frames::msg_asset_id,
    constants::ZERO_B256,
    context::msg_amount,
};

configurable {
    /// The privileged user with the ability to set assets for payment for the registry
    OWNER: Identity = Identity::Address(Address::from(ZERO_B256)),
}

storage {
    /// Cost rate per asset
    assets: StorageMap<ContractId, Option<u64>> = StorageMap {},
    /// A mapping of names to records
    names: StorageMap<str[8], Record> = StorageMap {},
}

// TODO: Change the static 8 length str with a dynamic string when possible
impl NameRegistry for Contract {
    #[payable]
    #[storage(read, write)]
    fn extend(name: str[8], duration: u64, payment_asset: ContractId) {
        let record = storage.names.get(name).try_read();
        require(record.is_some(), RegistrationValidityError::NameNotRegistered);

        let rate = storage.assets.get(payment_asset).try_read();

        require(msg_asset_id() == payment_asset && rate.unwrap().is_some(), AssetError::IncorrectAssetSent);
        require((duration / 100) * rate.unwrap().unwrap() <= msg_amount(), AssetError::InsufficientPayment);

        let mut record = record.unwrap();
        record.expiry = record.expiry + duration;

        storage.names.insert(name, record);

        log(RegistrationExtendedEvent {
            duration,
            name,
            new_expiry: record.expiry,
        });
    }

    #[payable]
    #[storage(read, write)]
    fn register(
        name: str[8],
        duration: u64,
        owner: Identity,
        identity: Identity,
        payment_asset: ContractId,
    ) {
        let record = storage.names.get(name).try_read();
        if record.is_some() {
            require(timestamp() > record.unwrap().expiry, RegistrationValidityError::NameNotExpired);
        }

        let rate = storage.assets.get(payment_asset).try_read();

        require(msg_asset_id() == payment_asset && rate.unwrap().is_some(), AssetError::IncorrectAssetSent);
        require((duration / 100) * rate.unwrap().unwrap() <= msg_amount(), AssetError::InsufficientPayment);

        let record = Record::new(timestamp() + duration, identity, owner);

        storage.names.insert(name, record);

        log(NameRegisteredEvent {
            expiry: record.expiry,
            name,
            owner,
            identity,
        });
    }

    #[storage(write)]
    fn set_asset(id: ContractId, rate: Option<u64>) {
        require(msg_sender().unwrap() == OWNER, AuthorizationError::SenderNotOwner);
        storage.assets.insert(id, rate);
        log(AssetRateEvent { id, rate });
    }

    #[storage(read, write)]
    fn set_identity(name: str[8], identity: Identity) {
        let record = storage.names.get(name).try_read();
        require(record.is_some(), RegistrationValidityError::NameNotRegistered);
        let previous_record = record.unwrap();
        require(timestamp() < previous_record.expiry, RegistrationValidityError::NameExpired);
        require(previous_record.owner == msg_sender().unwrap(), AuthorizationError::SenderNotOwner);

        let new_record = Record::new(previous_record.expiry, identity, previous_record.owner);

        storage.names.insert(name, new_record);

        log(IdentityChangedEvent {
            name,
            new_identity: new_record.identity,
            previous_identity: previous_record.identity,
        });
    }

    #[storage(read, write)]
    fn set_owner(name: str[8], owner: Identity) {
        let record = storage.names.get(name).try_read();
        require(record.is_some(), RegistrationValidityError::NameNotRegistered);
        let previous_record = record.unwrap();
        require(timestamp() < previous_record.expiry, RegistrationValidityError::NameExpired);
        require(previous_record.owner == msg_sender().unwrap(), AuthorizationError::SenderNotOwner);

        let new_record = Record::new(previous_record.expiry, previous_record.identity, owner);

        storage.names.insert(name, new_record);

        log(OwnerChangedEvent {
            name,
            new_owner: new_record.owner,
            previous_owner: previous_record.owner,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn expiry(name: str[8]) -> Result<u64, RegistrationValidityError> {
        match storage.names.get(name).try_read() {
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
    fn identity(name: str[8]) -> Result<Identity, RegistrationValidityError> {
        match storage.names.get(name).try_read() {
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
    fn owner(name: str[8]) -> Result<Identity, RegistrationValidityError> {
        match storage.names.get(name).try_read() {
            Option::Some(record) => {
                match timestamp() < record.expiry {
                    true => Result::Ok(record.owner),
                    false => Result::Err(RegistrationValidityError::NameExpired),
                }
            },
            Option::None => Result::Err(RegistrationValidityError::NameNotRegistered),
        }
    }

    #[storage(read)]
    fn rate(id: ContractId) -> Option<u64> {
        match storage.assets.get(id).try_read() {
            Option::Some(rate) => rate,
            Option::None => Option::None,
        }
    }
}
