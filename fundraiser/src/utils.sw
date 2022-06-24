library utils;

dep errors;

use std::{assert::require, chain::auth::{AuthError, msg_sender}, identity::Identity, result::*};

use errors::UserError;

pub fn sender_identity() -> Identity {
    // TODO: use msg_sender().unwrap() (and remove this function) when that becomes functional
    // Related: https://github.com/FuelLabs/sway/pull/1958
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}

pub fn validate_id(id: u64, count: u64) {
    require(id != 0 && id <= count, UserError::NoSuchCampaign);
}
