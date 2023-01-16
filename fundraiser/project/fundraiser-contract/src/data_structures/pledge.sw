library pledge;

/// Used to track the amount pledged by a user to a specific campaign
pub struct Pledge {
    /// The amount pledged to a campaign
    amount: u64,
    /// The unique identifier for the campaign
    campaign_id: u64,
}

impl Pledge {
    pub fn new(amount: u64, campaign_id: u64) -> Self {
        Self {
            amount,
            campaign_id,
        }
    }
}
