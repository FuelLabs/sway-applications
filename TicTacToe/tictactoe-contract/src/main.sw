contract;

mod data_structures;
mod errors;
mod events;
mod interface;
mod utils;

use core::ops::Eq;
use ::data_structures::State;
use ::errors::{GameStateError, PlayerError, PositionError, GameIDError};
use ::events::{GameDrawnEvent, GameWonEvent, NewGameEvent};
use ::interface::Game;
use std::{auth::msg_sender, hash::Hash, storage::storage_vec::*};
use ::utils::{draw, win_check};

pub struct GameData {
    /// Keeps track of the move counter for various checks (win, draw, etc.).
    move_counter: u64,
    /// The first player of the game.
    player_one: Option<Identity>,
    /// The current player turn.
    player_two: Option<Identity>,
    /// The second player of the game.
    player_turn: Option<Identity>,
    /// Keeps track of the game, its value is either Ended or Playing.
    state: State,
}

impl GameData {
    fn new(player_one: Identity, player_two: Identity) -> GameData {
        GameData {
            player_one: Some(player_one),
            player_two: Some(player_two),
            player_turn: Some(player_one),
            state: State::Playing,
            move_counter: 0
        }
    }
}

storage {
    /// Keeps track of all of the created games
    games: StorageVec<GameData> = StorageVec {},
    /// Keeps track of all boards.  Each board has a corresponding game.
    /// I did it this way because I wasn't able to add the board to game data.
    boards: StorageVec<StorageVec<Option<bool>>> = StorageVec {},
}

impl Game for Contract {
    #[storage(read, write)]
    fn new_game(player_one: Identity, player_two: Identity) -> u64 {
        let new_game = GameData::new(player_one, player_two);
        storage.games.push(new_game);
        storage.boards.push(StorageVec {});
        let mut new_board = storage.boards.get(0).unwrap();
        new_board.resize(9, None);

        log(NewGameEvent {
            player_one,
            player_two,
        });
        storage.games.len() - 1
    }

    #[storage(read, write)]
    fn make_move(position: u64, game_id: u64) {
        let game = storage.games.get(game_id).unwrap().read();
        let current_player = game.player_turn;

        // Only the current player may play
        require(
            current_player == msg_sender()
                .unwrap(),
            PlayerError::IncorrectPlayerTurn,
        );

        // This move has to be a valid choice on the board
        require(position < 9, PositionError::InvalidPosition);

        let mut board = storage.boards.get(game_id).unwrap().read();
        require(
            board
                .get(position)
                .unwrap()
                .try_read()
                .unwrap() == None,
            PositionError::CellIsNotEmpty,
        );

        let last_move_counter = game.move_counter;
        let is_player_one = last_move_counter % 2 == 0;

        // Make the move and update the board
        board.set(position, Some(is_player_one));
        storage.boards.set(game_id, board);

        // Update number of moves
        let current_move_counter = last_move_counter + 1;
        game.move_counter = current_move_counter;

        // Update the player
        //let current_player = storage.player_turn.read().unwrap();
        let player_one = game.player_one;
        let player_two = game.player_two;
        if (current_player == player_one) {
            game.player_turn = Some(player_two);
        } else {
            game.player_turn = Some(player_one);
        }

        // Detemine if there is a winner or if it is a draw
        if (current_move_counter > 4) {
            let board = board.load_vec();

            if win_check(board, is_player_one) {
                game.state = State::Ended;
                log(GameWonEvent {
                    player: msg_sender().unwrap(),
                });
            } else if draw(board, current_move_counter) {
                game.state = State::Ended;
                log(GameDrawnEvent {
                    player_one,
                    player_two,
                });
            }
        }
        storage.games.set(game_id, game);
    }

    #[storage(read)]
    fn get_board(game_id: u64) -> Vec<Option<bool>> {
        storage.boards.get(game_id).unwrap().read().load_vec()
    }

    #[storage(read)]
    fn get_game_state(game_id: u64) -> State {
        storage.games.get(game_id).unwrap().read().state
    }

    #[storage(read)]
    fn get_current_player(game_id: u64) -> Option<Identity> {
        let game = storage.games.get(game_id).unwrap().read();
        match game.state {
            State::Playing => {
                game.player_turn
            },
            State::Ended => {
                None
            }
        }
    }

    #[storage(read)]
    fn get_players(game_id: u64) -> Option<(Identity, Identity)> {
        let game = storage.games.get(game_id).unwrap().read();
        match game.state {
            State::Playing => {
                Some((game.player_one.unwrap(), game.player_two.unwrap()))
            },
            State::Ended => {
                None
            }
        }
    }

    #[storage(read)]
    fn get_move_counter(game_id: u64) -> u64 {
        let game = storage.games.get(game_id).unwrap().read();
        game.move_counter
    }
}
