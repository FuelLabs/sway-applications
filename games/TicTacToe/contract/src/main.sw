contract;

use core::*;
use std::{address::Address, chain::auth::msg_sender, hash::sha256, storage::{StorageMap, get, store}};

use data_structures::{Players, Winner, Game, core::ops::Eq};

storage {
    game: Game,
    player_turn: u64,
    map: StorageMap<u64,
    u64>, 
}

abi TicTacToe {
    #[storage(write)]fn new_game(player_one: Players, player_two: Players) -> Game;
    #[storage(read, write)]fn make_move(position: u64);
    #[storage(read, write)]fn next_player();
    #[storage(write)]fn end_game(game: Game) -> Winner;
    #[storage(read)]fn map_is_full() -> bool;
}

impl TicTacToe for Contract {
    #[storage(write)]
    fn new_game(player_one: Players, player_two: Players) -> Game {
        let mut game = Game {
            PlayerOne: player_one,
            PlayerTwo: player_two,
            winner: Winner::None,
        };
        storage.player_turn = 1;
        let mut counter = 0;
        while counter < 9 {
            counter += 1;
            insert_into_map(counter, 0)
        }
        return game;
    }

    // This function first checks who's turn it is and then inserts to the storage map the player key + the position
    #[storage(read, write)]
    fn make_move(position: u64) {
        if storage.player_turn == 1 {
            insert_into_map(1, position);
        }
        if storage.player_turn == 2 {
            insert_into_map(2, position);
        }
    }

    // This function first checks who's turn it is and then switches to the other one.
    #[storage(read, write)]
    fn next_player() {
        if storage.player_turn == 1 {
            storage.player_turn = 2;
        }
        if storage.player_turn == 2 {
            storage.player_turn = 1;
        }
    }

    //Check each cell. If one of them is empty (contains 0), then the map isn't full yet.
    #[storage(read)]
    fn map_is_full() -> bool {
        let mut counter = 0;
        let mut break_early = false;
        let mut result = true;
        while counter < 9 {
            if break_early == true {
                // here we ensure the condition will evaluate to false, breaking the loop
                counter = 10;
                result = false;
            }
            counter += 1;
            if get_from_map(counter) == 0 {
                break_early = true;
            }
        }
        return result;
    }

    // save the game and return the winner
    #[storage(write)]
    fn end_game(game: Game) -> Winner {
        storage.game = game;
        return game.winner;
    }
}

//save the player moves in the storage
#[storage(write)]
fn insert_into_map(key: u64, value: u64) {
    storage.map.insert(key, value);
}

//get the player moves in the storage
#[storage(read)]
fn get_from_map(key: u64) -> u64 {
    return storage.map.get(key);
}
