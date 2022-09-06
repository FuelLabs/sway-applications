library data_structures;
pub struct ClaimData {
    /// The amount the user has claimed
    amount: u64,
    /// Whether the user has claimed
    claimed: bool,
}
impl ClaimData {
    fn new(amount: u64, claimed: bool) -> Self {
        Self {
            amount,
            claimed,
        }
    }
}
