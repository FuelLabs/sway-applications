library data_structures;

use std::{identity::Identity, option::Option};

pub struct TokenMetaData {
    /// The user that is allowed to transfer this token.
    approved: Option<Identity>,
    /// The user that owns this token.
    owner: Identity,
}

impl TokenMetaData {
    fn new(approved: Option<Identity>, owner: Identity) -> Self {
        TokenMetaData {
            approved, owner
        }
    }
}
