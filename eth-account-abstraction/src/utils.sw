library utils;

use std::{
    hash::{
        keccak256,
        sha256,
    },
    identity::Identity,
};

//Applies the prefix used by Geth to a message hash.
//Returns the prefixed hash.
pub fn eth_prefix(msg_hash: b256) -> b256 {
    let prefix = "\x19Ethereum Signed Message:\n32";
    sha256((prefix, msg_hash))
}

//Creates an EIP-191 compliant transaction hash, of the version:
//0x45 personal sign.
pub fn eip_191_hash(
    to: Identity,
    value: u64,
    data: b256,
    nonce: u64
) -> b256 {
    let initial_byte= 0x19u8;
    let version_byte= 0x45u8;
    keccak256((
        initial_byte,
        version_byte,
        to, 
        value,
        data,
        nonce
    ))
}