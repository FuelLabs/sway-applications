library;

/// The state of a claim.
pub enum ClaimState {
    /// The claim is unclaimed.
    Unclaimed: (),
    /// The claim has been claimed for the given amount.
    Claimed: u64,
}

impl core::ops::Eq for ClaimState {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (ClaimState::Claimed(balance1), ClaimState::Claimed(balance2)) => {
                balance1 == balance2
            },
            (ClaimState::Unclaimed, ClaimState::Unclaimed) => true,
            _ => false,
        }
    }
}
