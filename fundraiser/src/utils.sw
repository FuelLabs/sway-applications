library utils;

dep data_structures;
dep errors;

use std::{
    assert::require,
    block::height,
    chain::auth::{AuthError, msg_sender},
    contract_id::ContractId,
    identity::Identity,
    result::*,
    revert::revert,
    token::{force_transfer_to_contract, transfer_to_output}
};

use errors::UserError;
use data_structures::Campaign;

pub fn sender_identity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}

pub fn transfer(to: Identity, amount: u64, asset: ContractId) {
    match to {
        Identity::Address(address) => {
            transfer_to_output(amount, asset, address);
        },
        Identity::ContractId(address) => {
            force_transfer_to_contract(amount, asset, address);
        }
    }
}

pub fn validate_id(id: u64, count: u64) {
    require(id != 0 && id <= count, UserError::NoSuchCampaign);
}

pub fn validate_sender(sender: Identity, other: Identity) {
    match sender {
        Identity::Address(sender) => {
            match other {
                Identity::Address(user) => require(sender.value == user.value, UserError::UnauthorizedUser), _ => revert(42), 
            }
        },
        Identity::ContractId(sender) => {
            match other {
                Identity::ContractId(user) => require(sender.value == user.value, UserError::UnauthorizedUser), _ => revert(42), 
            }
        }
    };
}
