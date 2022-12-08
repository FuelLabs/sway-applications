contract;

dep interface;
dep events;
dep errors;

use interface::Game;
use events::NewGameEvent;
use errors::Errors;
use core::ops::Eq;
use std::{logging::log, auth::msg_sender};

impl<T> Eq for Option<T> {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (Option::None, Option::None) => true,
            (Option::Some(T), Option::Some(T)) => true,
            _ => false,
        }
    }
}

storage {
    // Stores the players
    player_one: Option<Identity> = Option::None,
    player_two: Option<Identity> = Option::None,
    // Stores each player's moves,
    board: StorageMap<u64, Option<Identity>> = StorageMap {},
    // Stores move counter
    player_turn: Option<Identity> = Option::None,
}

impl Game for Contract {

    #[storage(write)]
    fn new_game(player_one: Identity, player_two: Identity) {
        storage.player_one = Option::Some(player_one);
        storage.player_two = Option::Some(player_two);

        storage.player_turn = Option::Some(player_one);
        let mut position = 0;
        while position < 9 {
            storage.board.insert(position, Option::None::<Identity>());
            position += 1;
        }
        log(NewGameEvent {player_one, player_two});
    }

    #[storage(read, write)]
    fn move(position: u64) {
        // check if game hasn't ended, if the cell is empty and that the right player is making the move
        require(storage.player_turn.unwrap() == msg_sender().unwrap(), Errors::IncorrectPlayerTurn);
        require(position < 9, Errors::InvalidPosition);
        require(storage.board.get(position) == Option::None::<Identity>(), Errors::CellIsNotEmpty);
        storage.board.insert(position, Option::Some(msg_sender().unwrap()));
        // Check if the game is over, if there is a winner or if it's a draw, emit an event saying a winner has been picked or if it's a Draw. In that case, the player_turn needs to be None. Otherwise, the game hasn't ended, then it's the other player to be set in the player_turn
    }
    
}
