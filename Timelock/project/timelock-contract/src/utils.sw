library utils;

use std::{bytes::Bytes, hash::sha256};

pub fn create_hash(
    recipient: Identity,
    value: Option<u64>,
    asset_id: Option<ContractId>,
    data: Bytes,
    timestamp: u64,
) -> b256 {
    sha256((recipient, value, asset_id, data, timestamp))
}
