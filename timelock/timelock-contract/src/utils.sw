library;

use ::data_structures::Asset;
use std::{bytes::Bytes, hash::{Hash, Hasher, sha256}};

impl Hash for Option<Bytes> {
    fn hash(self, ref mut state: Hasher) {
        match self {
            Some(bytes) => bytes.hash(state),
            None => 0.hash(state),
        }
    }
}

impl Hash for Option<Asset> {
    fn hash(self, ref mut state: Hasher) {
        match self {
            Some(asset) => {
                asset.amount.hash(state);
                asset.id.hash(state);
            },
            None => 0.hash(state),
        }
    }
}

/// Creates a transaction id as a hash of the transaction data.
///
/// # Arguments
///
/// * `recipient`: [Identity] - The recipient of the transaction.
/// * `asset`: [Option<Asset>] - The asset being transferred.
/// * `data`: [Option<Bytes>] - The data being transferred.
/// * `timestamp`: [u64] - The timestamp of the transaction.
///
/// # Returns
///
/// * [b256] - The transaction id.
pub fn create_hash(
    recipient: Identity,
    asset: Option<Asset>,
    data: Option<Bytes>,
    timestamp: u64,
) -> b256 {
    sha256((recipient, asset, data, timestamp))
}
