library;

use ::data_structures::campaign_state::CampaignState;

/// General data structure containing information about a campaign.
pub struct CampaignInfo {
    /// The user who has created the campaign.
    author: Identity,
    /// The asset that this campaign accepts as a deposit.
    asset: AssetId,
    /// The user to whom the funds will be sent to upon a successful campaign.
    beneficiary: Identity,
    // Whether the campaign is currently: Funding, Claimed, Cancelled.
    state: CampaignState,
    /// The end time for the campaign after which it becomes locked.
    deadline: u64,
    /// The amount needed to deem the campaign a success.
    target_amount: u64,
    /// The current amount pledged used to measure against the target_amount.
    total_pledge: u64,
}

impl CampaignInfo {
    /// Creates a new campaign.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset that this campaign accepts as a deposit.
    /// * `author`: [Identity] - The user who has created the campaign.
    /// * `beneficiary`: [Identity] - The user to whom the funds will be sent to upon a successful campaign.
    /// * `deadline`: [u64] - The end time for the campaign after which it becomes locked.
    /// * `target_amount`: [u64] - The amount needed to deem the campaign a success.
    ///
    /// # Returns
    ///
    /// * [CampaignInfo] - The newly created campaign.
    pub fn new(
        asset: AssetId,
        author: Identity,
        beneficiary: Identity,
        deadline: u64,
        target_amount: u64,
    ) -> Self {
        Self {
            asset,
            author,
            beneficiary,
            state: CampaignState::Funding,
            deadline,
            target_amount,
            total_pledge: 0,
        }
    }
}
