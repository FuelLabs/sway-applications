library;

use core::ops::Eq;

/// Represents the current state of the campaign.
pub enum CampaignState {
    /// The campaign has been cancelled.
    Cancelled: (),
    /// The campain was successful and the funds have been claimed.
    Claimed: (),
    /// The campaign is still accepting funds.
    Funding: (),
}

impl Eq for CampaignState {
    fn eq(self, other: CampaignState) -> bool {
        match (self, other) {
            (CampaignState::Cancelled, CampaignState::Cancelled) => true,
            (CampaignState::Claimed, CampaignState::Claimed) => true,
            (CampaignState::Funding, CampaignState::Funding) => true,
            _ => false,
        }
    }
}
