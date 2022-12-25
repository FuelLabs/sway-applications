library utils;

use std::{bytes::Bytes, hash::sha256};

pub fn create_hash(recipient: Identity, value: u64, data: Bytes, timestamp: u64) -> b256 {
    sha256((recipient, value, data, timestamp))
}
