library;

use core::ops::Eq;

pub enum State {
    NotInitialized: (),
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
