library;

use core::ops::Eq;

pub enum State {
    Playing: (),
    Ended: (),
}

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (State::Playing, State::Playing) => true,
            (State::Ended, State::Ended) => true,
            _ => false,
        }
    }
}
