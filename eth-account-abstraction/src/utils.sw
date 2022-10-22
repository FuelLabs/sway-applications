library utils;

use std::{
    hash::sha256,
};

//Applies the prefix used by Geth to a message hash
//Returns the prefixed hash
pub fn eth_prefix(msg_hash: b256) -> b256 {
    let prefix = "\x19Ethereum Signed Message:\n32";
    sha256((prefix, msg_hash))
}