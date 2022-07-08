contract;

use core::*;
use std::{
    address::Address,
    storage::{store, get, StorageMap},
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
    map: StorageMap<u64,u64>,
}

abi TicTacToe {
    #[storage(write)]fn new_game(player_one: Players, player_two: Players) -> Game;
    #[storage(read, write)]fn make_move(game: Game);
    fn end_game(game: Game) -> Winners;
    #[storage(read)]fn map_is_full() -> bool;
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
        let mut counter = 0;
        while counter < 9 {
            counter += 1;
            insert_into_map(counter, 0)
        }
        return game;
    }

    #[storage(read, write)]fn make_move(game: Game) {
         
    }

    //Check each cell. If one of them is empty (contains 0), then the map isn't full yet.
    #[storage(read)]fn map_is_full() -> bool {
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
            if get_from_map(counter) ==0 {
                break_early = true;
            }
        }
        return result;
    }

    fn end_game(game: Game) -> Winners{
        return game.winner;
    }
}

#[storage(write)]fn insert_into_map(key: u64, value: u64) {
    storage.map.insert(key, value);
}

#[storage(read)]fn get_from_map(key: u64) -> u64{
    return storage.map.get(key);
}