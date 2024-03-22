library;

/// Used to track the amount pledged by a user to a specific campaign.
pub struct Pledge {
    /// The amount pledged to a campaign.
    amount: u64,
    /// The unique identifier for the campaign.
    campaign_id: u64,
}

impl Pledge {
    /// Creates a new pledge.
    ///
    /// # Arguments
    ///
    /// * `amount`: [u64] - The amount pledged to a campaign.
    /// * `campaign_id`: [u64] - The unique identifier for the campaign.
    ///
    /// # Returns
    ///
    /// * [Pledge] - The new pledge.
    pub fn new(amount: u64, campaign_id: u64) -> Self {
        Self {
            amount,
            campaign_id,
        }
    }
}
