contract;

dep data_structures;
dep errors;
dep events;
dep interface;

use data_structures::Record;
use errors::{AssetError, AuthorisationError, RegistrationValidityError};
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
    context::call_frames::msg_asset_id,
    context::msg_amount,
    logging::log,
    storage::StorageMap,
};

storage {
    /// A mapping of names to an option of records, with a none representing an unregistered name
    names: StorageMap<str[8], Option<Record>> = StorageMap {},
}

const ASSET_ID = ~ContractId::from(ASSET_B256);

// TODO: Change the static 8 length str with a dynamic string when possible
impl NameRegistry for Contract {
    #[storage(read, write)]
    fn extend(name: str[8], duration: u64) {
        require(storage.names.get(name).is_some(), RegistrationValidityError::NameNotRegistered);        
        require(msg_asset_id() == ASSET_ID, AssetError::IncorrectAssetSent);
        require((duration / 100) * PRICE_PER_HUNDRED <= msg_amount(), AssetError::InsufficientPayment);

        let mut record = storage.names.get(name).unwrap();
        record.expiry = record.expiry + duration;

        storage.names.insert(name, Option::Some(record));

        log(RegistrationExtendedEvent {
            duration,
            name,
            new_expiry: record.expiry,
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
            require(timestamp() > record.expiry, RegistrationValidityError::NameNotExpired);
        }

        require(msg_asset_id() == ASSET_ID, AssetError::IncorrectAssetSent);
        require((duration / 100) * PRICE_PER_HUNDRED <= msg_amount(), AssetError::InsufficientPayment);
        
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
        require(storage.names.get(name).is_some(), RegistrationValidityError::NameNotRegistered);
        let previous_record = storage.names.get(name).unwrap();
        require(timestamp() < previous_record.expiry, RegistrationValidityError::NameExpired);
        require(previous_record.owner == msg_sender().unwrap(), AuthorisationError::SenderNotOwner);

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
    fn set_owner(name: str[8], owner: Identity) {
        require(storage.names.get(name).is_some(), RegistrationValidityError::NameNotRegistered);
        let previous_record = storage.names.get(name).unwrap();
        require(timestamp() < previous_record.expiry, RegistrationValidityError::NameExpired);
        require(previous_record.owner == msg_sender().unwrap(), AuthorisationError::SenderNotOwner);

        let new_record = Record {
            expiry: previous_record.expiry,
            identity: previous_record.identity,
            owner,
        };

        storage.names.insert(name, Option::Some(new_record));

        log(OwnerChangedEvent {
            name,
            new_owner: new_record.owner,
            previous_owner: previous_record.owner,
        });
    }

    #[storage(read)]
    fn expiry(name: str[8]) -> Result<u64, RegistrationValidityError> {
        let res = match storage.names.get(name) {
            Option::Some(record) => {
                match timestamp() < record.expiry {
                    true => {
                        Result::Ok(record.expiry)
                    },
                    false => {
                        Result::Err(RegistrationValidityError::NameExpired)
                    }
                }
            },
            Option::None => {
                Result::Err(RegistrationValidityError::NameNotRegistered)
            }
        };
        res
    }

    #[storage(read)]
    fn identity(name: str[8]) -> Result<Identity, RegistrationValidityError> {
        let res = match storage.names.get(name) {
            Option::Some(record) => {
                match timestamp() < record.expiry {
                    true => {
                        Result::Ok(record.identity)
                    },
                    false => {
                        Result::Err(RegistrationValidityError::NameExpired)
                    }
                }
            },
            Option::None => {
                Result::Err(RegistrationValidityError::NameNotRegistered)
            }
        };
        res
    }

    #[storage(read)]
    fn owner(name: str[8]) -> Result<Identity, RegistrationValidityError> {
        let res = match storage.names.get(name) {
            Option::Some(record) => {
                match timestamp() < record.expiry {
                    true => {
                        Result::Ok(record.owner)
                    },
                    false => {
                        Result::Err(RegistrationValidityError::NameExpired)
                    }
                }
            },
            Option::None => {
                Result::Err(RegistrationValidityError::NameNotRegistered)
            }
        };
        res
    }
}
