library campaign_info;

dep campaign_state;

use campaign_state::CampaignState;

/// General data structure containing information about a campaign
pub struct CampaignInfo {
    /// The user who has created the campaign
    author: Identity,
    /// The asset that this campaign accepts as a deposit
    asset: ContractId,
    /// The user to whom the funds will be sent to upon a successful campaign
    beneficiary: Identity,
    // Whether the campaign is currently: Funding, Claimed, Cancelled
    state: CampaignState,
    /// The end time for the campaign after which it becomes locked
    deadline: u64,
    /// The amount needed to deem the campaign a success
    target_amount: u64,
    /// The current amount pledged used to measure against the target_amount
    total_pledge: u64,
}

impl CampaignInfo {
    pub fn new(
        asset: ContractId,
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
