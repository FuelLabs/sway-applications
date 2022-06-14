library utils;

dep errors;

use errors::InputError;
use std::{
    assert::require, 
    chain::auth::{AuthError, msg_sender},
    constants::NATIVE_ASSET_ID,
    identity::Identity, 
    result::*,
};

// This function will take two identities and return true if they are the same
pub fn identities_equal(identity1: Identity, identity2: Identity) -> bool {
    match identity1 {
        Identity::Address(identity1) => {
            match identity2 {
                Identity::Address(identity2) => identity1.value == identity2.value,
                _ => false,
            }
        },
        Identity::ContractId(identity1) => {
            match identity2 {
                Identity::ContractId(identity2) => identity1.value == identity2.value,
                _ => false,
            }
        }
    }
}

pub fn sender_identity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}

// This function will panic if the given Identity points to the zero value
pub fn validate_identity(entity: Identity) {
    let address = match entity {
        Identity::Address(entity) => entity.value,
        Identity::ContractId(entity) => entity.value,
    };
    
    require(address != NATIVE_ASSET_ID, InputError::InputAddressCannotBeZero);
}
