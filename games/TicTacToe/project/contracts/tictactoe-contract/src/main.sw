contract;

mod data_structures;
mod errors;
mod events;
mod interface;
mod utils;

use core::ops::Eq;
use ::data_structures::State;
use ::errors::{GameStateError, PlayerError, PositionError};
use ::events::{GameDrawnEvent, GameWonEvent, NewGameEvent};
use ::interface::Game;
use std::{auth::msg_sender, hash::Hash, storage::storage_vec::*};
use ::utils::{draw, win_check};

storage {
    /// Keeps track of each player move.
    board: StorageVec<Option<Identity>> = StorageVec {},
    /// Keeps track of the move counter for various checks (win, draw, etc.).
    move_counter: u64 = 0,
    /// The first player of the game.
    player_one: Option<Identity> = None,
    /// The current player turn.
    player_turn: Option<Identity> = None,
    /// The second player of the game.
    player_two: Option<Identity> = None,
    /// Keeps track of the game, its value is either Ended or Playing.
    state: State = State::Ended,
}

impl Game for Contract {
    #[storage(read, write)]
    fn new_game(player_one: Identity, player_two: Identity) {
        require(
            storage
                .state
                .read() == State::Ended,
            GameStateError::GameHasNotEnded,
        );

        storage.player_one.write(Some(player_one));
        storage.player_two.write(Some(player_two));
        storage.player_turn.write(Some(player_one));

        // Once a game has been played we need to reset all values.
        storage.board.resize(9, None);
        storage.board.fill(None);
        storage.move_counter.write(0);
        storage.state.write(State::Playing);

        log(NewGameEvent {
            player_one,
            player_two,
        });
    }

    #[storage(read, write)]
    fn make_move(position: u64) {
        // Ensure the game is active
        require(
            storage
                .state
                .read() == State::Playing,
            GameStateError::GameHasEnded,
        );

        // Only the current player may play
        let current_player = storage.player_turn.read().unwrap();
        require(
            current_player == msg_sender()
                .unwrap(),
            PlayerError::IncorrectPlayerTurn,
        );

        // This move has to be a valid choice on the board
        require(position < 9, PositionError::InvalidPosition);
        require(
            storage
                .board
                .get(position)
                .unwrap()
                .try_read()
                .unwrap() == None,
            PositionError::CellIsNotEmpty,
        );

        // Make the move and update the board
        storage.board.set(position, Some(msg_sender().unwrap()));

        // Update number of moves
        let last_move_counter = storage.move_counter.read();
        storage.move_counter.write(last_move_counter + 1);
        let current_move_counter = last_move_counter + 1;

        // Update the player
        let current_player = storage.player_turn.read().unwrap();
        let player_one = storage.player_one.read().unwrap();
        let player_two = storage.player_two.read().unwrap();
        if (current_player == player_one) {
            storage.player_turn.write(Some(player_two));
        } else {
            storage.player_turn.write(Some(player_one));
        }

        // Detemine if there is a winner or if it is a draw
        if (current_move_counter > 4) {
            let mut board = storage.board.load_vec();

            if win_check(board, current_player) {
                storage.state.write(State::Ended);
                log(GameWonEvent {
                    player: msg_sender().unwrap(),
                });
            } else if draw(board, player_one, player_two, current_move_counter)
            {
                storage.state.write(State::Ended);
                log(GameDrawnEvent {
                    player_one,
                    player_two,
                });
            }
        }
    }

    #[storage(read)]
    fn get_board() -> Vec<Option<Identity>> {
        storage.board.load_vec()
    }
}
