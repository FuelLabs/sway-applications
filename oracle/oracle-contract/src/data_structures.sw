library;

use core::ops::Eq;

/// Represents the state of the oracle.
pub enum State {
    /// The oracle has not been initialized.
    NotInitialized: (),
    /// The oracle has been initialized.
    Initialized: (),
}

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (State::Initialized, State::Initialized) => true,
            (State::NotInitialized, State::NotInitialized) => true,
            _ => false,
        }
    }
}
