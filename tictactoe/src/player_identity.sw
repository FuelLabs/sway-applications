library player_identity;

use ::address::Address;

pub enum Players {
    None: (),
    PlayerOne: Address,
    PlayerTwo: Address,
}

impl core::ops::Eq for Players {
    fn eq(self, other: Self) -> bool {
        match(self, other) {
            (Players::PlayerOne(address1), Players::PlayerTwo(address2)) => {
                address1 == address2
            },
            (Players::PlayerTwo(address1), Players::PlayerOne(address2)) => {
                address1 == address2
            },
            _ => {
                false
            },
        }
    }
}

impl core::ops::Neq for Players {
    fn neq(self, other: Self) -> bool {
        match(self, other) {
            (Players::PlayerOne(address1), Players::PlayerTwo(address2)) => {
                address1 != address2
            },
            (Players::PlayerTwo(address1), Players::PlayerOne(address2)) => {
                address1 != address2
            },
            _ => {
                false
            },
        }
    }
}