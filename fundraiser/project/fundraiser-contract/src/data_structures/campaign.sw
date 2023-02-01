library campaign;

/// Used to track the campaigns that a user has created
pub struct Campaign {
    /// The unique identifier for the campaign
    id: u64,
}

impl Campaign {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
