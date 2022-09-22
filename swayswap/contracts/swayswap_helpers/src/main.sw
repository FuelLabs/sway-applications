library swayswap_helpers;

use std::{
    address::*,
    block::*,
    chain::auth::*,
    context::{*, call_frames::*},
    result::*,
    revert::revert,
    identity::Identity,
};

#[storage(read)] pub fn get_b256(key: b256) -> b256 {
    asm(r1: key, r2) {
        move r2 sp;
        cfei i32;
        srwq r2 r1;
        r2: b256
    }
}

// Store b256 values on memory
#[storage(write)] pub fn store_b256(key: b256, value: b256) {
    asm(r1: key, r2: value) {
        swwq r1 r2;
    };
}

/// Return the sender as an Address or panic
pub fn get_msg_sender_address_or_panic() -> Address {
    let sender: Result<Identity, AuthError> = msg_sender();
    if let Identity::Address(address) = sender.unwrap() {
       address
    } else {
       revert(0);
    }
}
