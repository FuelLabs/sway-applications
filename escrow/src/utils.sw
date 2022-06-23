library utils;

use std::{
    chain::auth::{AuthError, msg_sender},
    identity::Identity,
    result::*,
};

pub fn sender_identity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}
