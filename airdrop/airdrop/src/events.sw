library events;

use std::identity::Identity;

pub struct ClaimEvent {
    to: Identity,
    amount: u64,
}
