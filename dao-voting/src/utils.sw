library utils;

use std::{chain::auth::{AuthError, msg_sender}, identity::Identity, result::*};

pub fn sender_identity() -> Identity {
    // TODO: use msg_sender().unwrap() (and remove this function) when that becomes functional
    // Related: https://github.com/FuelLabs/sway/pull/1958
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}