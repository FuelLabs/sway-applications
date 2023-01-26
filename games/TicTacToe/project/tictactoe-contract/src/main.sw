contract;

dep interface;
dep events;
dep errors;
dep utils;
dep data_structures;

use interface::Game;
use events::{GameDrawnEvent, GameWonEvent, NewGameEvent};
use errors::{GameStateError, PlayerError, PositionError};
use utils::{draw, win_check};
use data_structures::State;
use core::ops::Eq;
use std::{auth::msg_sender, logging::log};

// This is needed for comparing the position when the cell is not empty.
// We only need to check if there is an Identity in the cell but we don't care about its value.
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
    /// Keeps track of each player move.
    board: StorageMap<u64, Option<Identity>> = StorageMap {},
    /// Keeps track of the move counter for various checks (win, draw, etc.).
    move_counter: u64 = 0,
    /// The first player of the game.
    player_one: Option<Identity> = Option::None,
    /// The current player turn.
    player_turn: Option<Identity> = Option::None,
    /// The second player of the game.
    player_two: Option<Identity> = Option::None,
    /// Keeps track of the game, its value is either Ended or Playing.
    state: State = State::Ended,
}

impl Game for Contract {
    #[storage(read, write)]
    fn new_game(player_one: Identity, player_two: Identity) {
        require(storage.state == State::Ended, GameStateError::GameHasNotEnded);

        storage.player_one = Option::Some(player_one);
        storage.player_two = Option::Some(player_two);
        storage.player_turn = Option::Some(player_one);

        // Once a game has been played we need to reset all values.
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
        require(storage.state == State::Playing, GameStateError::GameHasEnded);
        require(storage.player_turn.unwrap() == msg_sender().unwrap(), PlayerError::IncorrectPlayerTurn);
        require(position < 9, PositionError::InvalidPosition);
        require(storage.board.get(position) == Option::None::<Identity>(), PositionError::CellIsNotEmpty);

        storage.board.insert(position, Option::Some(msg_sender().unwrap()));
        storage.move_counter += 1;

        let current_player = storage.player_turn.unwrap();
        if (storage.player_turn.unwrap() == storage.player_one.unwrap())
        {
            storage.player_turn = storage.player_two;
        } else {
            storage.player_turn = storage.player_one;
        }

        if (storage.move_counter > 4) {
            let mut board = Vec::with_capacity(8);
            let mut i = 0;
            // We make a hard copy of the board to access the storage in an external function
            // because we cannot pass in storage references.
            // https://github.com/FuelLabs/sway/issues/3043
            while (i < 9) {
                board.push(storage.board.get(i));
                i += 1;
            }
            if (win_check(board, current_player)) {
                storage.player_turn = Option::None;
                storage.state = State::Ended;
                log(GameWonEvent {
                    player: msg_sender().unwrap(),
                });
            } else if draw(board, storage.player_one.unwrap(), storage.player_two.unwrap(), storage.move_counter)
            {
                storage.player_turn = Option::None;
                storage.state = State::Ended;
                log(GameDrawnEvent {
                    player_one: storage.player_one.unwrap(),
                    player_two: storage.player_two.unwrap(),
                });
            }
        }
    }
}
