library utils;

use std::{
    chain::auth::{AuthError, msg_sender},
    identity::Identity, 
    result::*,
};

/// Returns the `Identity` of the transaction's sender
pub fn sender_identity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}
