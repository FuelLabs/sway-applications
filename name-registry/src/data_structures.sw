library data_structures;

use std::identity::Identity;

pub struct Record {
    expiry: u64,
    identity: Identity,
    owner: Identity, 
}