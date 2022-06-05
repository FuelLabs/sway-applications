library events;

use std::chain::auth::Sender;

pub struct PledgedEvent {
    user: Sender,
    amount: u64,
    campaign_identifier: u64
}

pub struct UnpledgedEvent {
    user: Sender,
    amount: u64,
    campaign_identifier: u64
}

pub struct ClaimedEvent {
    user: Sender,
    amount: u64,
    campaign_identifier: u64
}
