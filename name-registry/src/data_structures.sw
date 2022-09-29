library data_structures;

use std::identity::Identity;

pub struct Record {
    owner: Identity,
    identity: Identity,
    expiry: u64,
}