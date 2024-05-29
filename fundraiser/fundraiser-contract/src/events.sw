library;

use ::data_structures::campaign_info::CampaignInfo;

/// Event for when a campaign is cancelled.
pub struct CancelledCampaignEvent {
    /// The unique identifier for the campaign.
    pub campaign_id: u64,
}

/// Event for when the proceeds of a campaign are claimed.
pub struct ClaimedEvent {
    /// The unique identifier for the campaign.
    pub campaign_id: u64,
}

/// Event for when a campaign is created.
pub struct CreatedCampaignEvent {
    /// The user who has created the campaign.
    pub author: Identity,
    /// Information about the entire campaign.
    pub campaign_info: CampaignInfo,
    /// The unique identifier for the campaign.
    pub campaign_id: u64,
}

/// Event for when a donation is made to a campaign.
pub struct PledgedEvent {
    /// The amount pledged.
    pub amount: u64,
    /// The unique identifier for the campaign.
    pub campaign_id: u64,
    /// The user who has pledged.
    pub user: Identity,
}

/// Event for when a donation is withdrawn from a campaign.
pub struct UnpledgedEvent {
    /// The amount unpledged.
    pub amount: u64,
    /// The unique identifier for the campaign.
    pub campaign_id: u64,
    /// The user who has unpledged.
    pub user: Identity,
}
