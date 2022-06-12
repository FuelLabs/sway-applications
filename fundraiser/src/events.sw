library events;

dep data_structures;

use std::identity::Identity;
use data_structures::Campaign;

pub struct CancelledCampaignEvent {
    id: u64,
}

pub struct ClaimedEvent {
    id: u64,
}

pub struct CreatedCampaignEvent {
    campaign: Campaign,
    id: u64,
}

pub struct PledgedEvent {
    amount: u64,
    id: u64,
}

pub struct UnpledgedEvent {
    amount: u64,
    id: u64,
}
