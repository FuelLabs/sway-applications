library state;

use core::ops::Eq;

pub enum State {
    Cancelled: (),
    Claimed: (),
    Funding: (),
}

impl Eq for State {
    fn eq(self, other: State) -> bool {
        match (self, other) {
            (State::Cancelled, State::Cancelled) => true,
            (State::Claimed, State::Claimed) => true,
            (State::Funding, State::Funding) => true,
            _ => false,
        }
    }
}
