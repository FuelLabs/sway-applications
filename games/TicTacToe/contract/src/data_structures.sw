library data_structures;

use std::{address::Address, option::Option};

// A game is defined by the Players and the winner.
pub struct Game {
    PlayerOne: Address,
    PlayerTwo: Address,
    winner: Option<Address>,
}    
