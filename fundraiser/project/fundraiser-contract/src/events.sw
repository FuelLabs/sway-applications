library events;

dep data_structures;

use data_structures::CampaignInfo;

pub struct CancelledCampaignEvent {
    /// The unique identifier for the campaign
    id: u64,
}

pub struct ClaimedEvent {
    /// The unique identifier for the campaign
    id: u64,
}

pub struct CreatedCampaignEvent {
    /// The user who has created the campaign
    author: Identity,
    /// Information about the entire campaign
    campaign_info: CampaignInfo,
    /// The unique identifier for the campaign
    id: u64,
}

pub struct PledgedEvent {
    /// The amount pledged
    amount: u64,
    /// The unique identifier for the campaign
    id: u64,
    /// The user who has pledged
    user: Identity,
}

pub struct UnpledgedEvent {
    /// The amount unpledged
    amount: u64,
    /// The unique identifier for the campaign
    id: u64,
    /// The user who has unpledged
    user: Identity,
}
