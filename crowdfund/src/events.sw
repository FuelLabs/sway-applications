library events;

use std::chain::auth::Sender;

pub struct PledgedEvent {
    user: Sender,
    amount: u64
}

pub struct UnpledgedEvent {
    user: Sender,
    amount: u64
}
