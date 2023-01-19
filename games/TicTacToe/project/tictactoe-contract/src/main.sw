contract;

dep interface;
dep events;
dep errors;
dep utils;
dep data_structures;

use interface::Game;
use events::{GameDrawnEvent, GameWonEvent, NewGameEvent};
use errors::Errors;
use utils::{draw, win_check};
use data_structures::State;
use core::ops::Eq;
use std::{auth::msg_sender, logging::log};

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
    // Stores make_move counter
    player_turn: Option<Identity> = Option::None,
    move_counter: u64 = 0,
    state: State = State::Ended,
}

impl Game for Contract {
    #[storage(read, write)]
    fn new_game(player_one: Identity, player_two: Identity) {
        require(storage.state == State::Ended, Errors::GameHasNotEnded);
        storage.player_one = Option::Some(player_one);
        storage.player_two = Option::Some(player_two);

        storage.player_turn = Option::Some(player_one);
        let mut position = 0;
        while position < 9 {
            storage.board.insert(position, Option::None::<Identity>());
            position += 1;
        }
        storage.move_counter = 0;
        storage.state = State::Playing;
        log(NewGameEvent {
            player_one,
            player_two,
        });
    }

    #[storage(read, write)]
    fn make_move(position: u64) {
        require(storage.state == State::Playing, Errors::GameHasEnded);
        // check if game hasn't ended, if the cell is empty and that the right player is making the make_move
        require(storage.player_turn.unwrap() == msg_sender().unwrap(), Errors::IncorrectPlayerTurn);
        require(position < 9, Errors::InvalidPosition);
        require(storage.board.get(position) == Option::None::<Identity>(), Errors::CellIsNotEmpty);
        storage.board.insert(position, Option::Some(msg_sender().unwrap()));
        storage.move_counter += 1;
        if (storage.move_counter > 4) {
            let mut board = Vec::with_capacity(8);
            let mut i = 0;
            //hard copy of the board
            while (i < 9) {
                board.push(storage.board.get(i));
                i += 1;
            }
            if (win_check(board, storage.player_turn)) {
                storage.player_turn = Option::None;
                storage.state = State::Ended;
                log(GameWonEvent {
                    player: msg_sender().unwrap(),
                });
            } else if draw(board, storage.player_one, storage.player_two) {
                storage.player_turn = Option::None;
                storage.state = State::Ended;
                log(GameDrawnEvent {
                    player_one: storage.player_one.unwrap(),
                    player_two: storage.player_two.unwrap(),
                });
            } else {
                if (storage.player_turn == storage.player_one) {
                    storage.player_turn = storage.player_two;
                } else {
                    storage.player_turn = storage.player_one;
                }
            }
        } else {
            if (storage.player_turn == storage.player_one) {
                storage.player_turn = storage.player_two;
            } else {
                storage.player_turn = storage.player_one;
            }
        }
    }
}
