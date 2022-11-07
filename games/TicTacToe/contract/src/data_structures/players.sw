library players;

use std::constants::ZERO_B256;

pub struct Players {
    player_one: Identity,
    player_two: Identity,
}

impl Players {
    pub fn default() -> Self {
        Self {
            player_one: Identity::Address(Address::from(ZERO_B256)),
            player_two: Identity::Address(Address::from(ZERO_B256)),
        }
    }

    pub fn new(player_one: Identity, player_two: Identity) -> Self {
        Self {
            player_one,
            player_two,
        }
    }
}