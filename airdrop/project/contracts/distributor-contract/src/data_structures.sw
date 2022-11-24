library data_structures;

pub struct ClaimData {
    /// The amount of an asset the user has claimed
    amount: u64,
    /// Whether the user has claimed
    claimed: bool,
}

impl ClaimData {
    pub fn new(amount: u64, claimed: bool) -> Self {
        Self {
            amount,
            claimed,
        }
    }
}
