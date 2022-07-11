library events;

use std::{identity::Identity};

pub struct ClaimEvent {
    amount: u64,
    to: Identity,
}

pub struct InitializeEvent {
    end_block: u64,
    merkleRoot: b256,
}
