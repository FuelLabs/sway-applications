library events;

dep data_structures;

use std::identity::Identity;
use data_structures::Campaign;

pub struct CancelledEvent {
    id: u64,
    user: Identity,
}

pub struct ClaimedEvent {
    amount: u64,
    id: u64,
    user: Identity,
}

pub struct CreatedCampaign {
    campaign: Campaign,
    id: u64,
}

pub struct PledgedEvent {
    amount: u64,
    id: u64,
    user: Identity,
}

pub struct UnpledgedEvent {
    amount: u64,
    id: u64,
    user: Identity,
}
