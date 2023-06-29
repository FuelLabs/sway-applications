library;

use ::data_structures::Asset;
use std::{bytes::Bytes, hash::sha256};

pub fn create_hash(
    recipient: Identity,
    asset: Option<Asset>,
    data: Option<Bytes>,
    timestamp: u64,
) -> b256 {
    sha256((recipient, asset, data, timestamp))
}
