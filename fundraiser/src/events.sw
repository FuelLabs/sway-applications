library events;

dep data_structures;

use std::identity::Identity;
use data_structures::Campaign;

pub struct PledgedEvent {
    user: Identity,
    amount: u64,
    id: u64
}

pub struct UnpledgedEvent {
    user: Identity,
    amount: u64,
    id: u64
}

pub struct ClaimedEvent {
    user: Identity,
    amount: u64,
    id: u64
}

pub struct CancelledEvent {
    user: Identity,
    id: u64
}

pub struct CreatedCampaign {
    campaign: Campaign,
    id: u64
}
