library campaign_state;

use core::ops::Eq;

pub enum CampaignState {
    Cancelled: (),
    Claimed: (),
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
