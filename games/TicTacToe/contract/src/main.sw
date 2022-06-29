contract;

use core::*;
use std::{
    address::Address,
    storage::{store,get},
    hash::sha256,
    chain::auth::msg_sender,
};

use player_identity::{Players, core::ops::Eq};

enum Winners {
    Player: Players,
    None: (),
    Draw: (),
}

struct Game {
    PlayerOne: Players,
    PlayerTwo: Players,
    winner: Winners,
    playerTurn: Players,
}
storage {
    game: Game,
}

abi TicTacToe {
    #[storage(write)]
    fn new_game(player_one: Players, player_two: Players) -> Game;
}

impl TicTacToe for Contract {

    #[storage(write)]
    fn new_game(player_one: Players, player_two: Players) -> Game {
        let mut game = Game {
            PlayerOne: player_one,
            PlayerTwo: player_two,
            winner: Winners::None,
            playerTurn: player_one,
        };
        storage.game = game;
        return game;
    }

    

}
