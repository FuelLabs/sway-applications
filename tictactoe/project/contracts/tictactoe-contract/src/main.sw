contract;

dep data_structures;
dep errors;
dep events;
dep interface;

use data_structures::GameState;
use events::*;
use errors::{Error};
use interface::TicTacToe;
use std::constants::ZERO_B256;
use std::auth::msg_sender;
use std::logging::log;

storage {
    player_turn: u64 = 1,
    player_one: Identity = Identity::Address(Address::from(ZERO_B256)),
    player_two: Identity = Identity::Address(Address::from(ZERO_B256)),
    board: StorageMap<u64, u64> = StorageMap {},
    game_state: GameState = GameState::InProgress,
}

impl TicTacToe for Contract {
    #[storage(write)]
    fn new_game(player_one: Identity, player_two: Identity) {
        storage.player_turn = 1;
        storage.player_one = player_one;
        storage.player_two = player_two;
        storage.game_state = GameState::InProgress;
        // clear the board
        let mut position = 0;
        while position < 9 {
            storage.board.insert(position, 0);
            position += 1;
        }
        log(NewGameEvent{player_one, player_two});
    }

    #[storage(read, write)]
    fn play_move(position: u64) {
        require(storage.game_state == GameState::InProgress, Error::GameIsOver);

        let player_id = if storage.player_turn == 1 {storage.player_one} else {storage.player_two};
        require(msg_sender().unwrap() == player_id, Error::NotYourTurn);

        require(storage.board.get(position) == 0, Error::PositionAlreadyTaken);
        require(position < 9, Error::InvalidPosition);

        storage.board.insert(position, storage.player_turn);
        log(MoveEvent{player: storage.player_turn, player_id, position});

        // Compute game state
        let winning_combinations = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        let mut combo_index = 0;
        while combo_index < 8 {
            let combination = winning_combinations[combo_index];
            let mut player_one_won = true;
            let mut player_two_won = true;

            let mut position_index = 0;
            while position_index < 3 {
                let position = combination[position_index];
                let player = storage.board.get(position);
                if player != 1 {
                    player_one_won = false;
                }
                if player != 2 {
                    player_two_won = false;
                }

                position_index += 1;
            }

            if player_one_won {
                storage.game_state = GameState::PlayerOneWon;
            }
            if player_two_won {
                storage.game_state = GameState::PlayerTwoWon;
            }

            combo_index += 1;
        }

        // Check if game is a draw
        let mut draw = true;
        let position_index = 0;
        while position_index < 9 {
            if storage.board.get(position_index) == 0 {
                draw = false;
            }
        }
        if draw {
            storage.game_state = GameState::Draw;
        }

        if storage.game_state != GameState::InProgress {
            log(GameOverEvent{game_state: storage.game_state});
        }

        // Switch player turn
        storage.player_turn = if storage.player_turn == 1 {2} else {1};
    }
}
