library data_structures;

use std::identity::Identity;

pub struct Claim {
    identity: Identity,
    amount: u64,
}

pub enum State {
    Initalized: (),
    NotInitalized: (),
}

impl core::ops::Eq for State {
    fn eq(self, other: Self) -> bool {
        match(self, other) {
            (State::Initalized, State::Initalized) => true, (State::NotInitalized, State::NotInitalized) => true, _ => false, 
        }
    }
}
