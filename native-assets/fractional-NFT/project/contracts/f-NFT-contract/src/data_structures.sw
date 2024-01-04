library;

pub enum State {
    Uninitialized: (),
    Distribution: (),
    Buyback: (),
    Withdrawn: (),
}

impl core::ops::Eq for State {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (Self::Uninitialized, Self::Uninitialized) => true,
            (Self::Distribution, Self::Distribution) => true,
            (Self::Buyback, Self::Buyback) => true,
            (Self::Withdrawn, Self::Withdrawn) => true,
            _ => false,
        }
    }
}
