library state;

use core::ops::Eq;

pub enum State {
    Funding: (),
    Cancelled: (),
    Claimed: (),
}

impl Eq for State {
    fn eq(self, other: State) -> bool {
        match (self, other) {
            (State::Funding, State::Funding) => true,
            (State::Cancelled, State::Cancelled) => true,
            (State::Claimed, State::Claimed) => true,
            _ => false,
        }
    }
}
