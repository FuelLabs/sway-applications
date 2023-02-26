library data_structures;

pub enum ClaimState {
    Unclaimed: (),
    Claimed: u64,
}

impl core::ops::Eq for ClaimState {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (ClaimState::Claimed(owner1), ClaimState::Claimed(owner2)) => {
                owner1 == owner2
            },
            (ClaimState::Unclaimed, ClaimState::Unclaimed) => true,
            _ => false,
        }
    }
}
