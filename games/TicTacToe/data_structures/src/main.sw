library data_structures;

use std::address::Address;

pub enum Players {
    None: (),
    PlayerOne: Address,
    PlayerTwo: Address,
}

// The Winner state is set to None during the game until it's over. Then it's either a Draw or one of the Players
pub enum Winner {
    Player: Players,
    None: (),
    Draw: (),
}

// A game is defined by the Players and the winner.
pub struct Game {
    PlayerOne: Players,
    PlayerTwo: Players,
    winner: Winner,
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
    

    
