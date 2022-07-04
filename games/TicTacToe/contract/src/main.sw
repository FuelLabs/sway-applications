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
    //grid: [u8;9],
}
storage {
    game: Game,
    map: StorageMap<u64,u64>,
}

abi TicTacToe {
    #[storage(write)]fn new_game(player_one: Players, player_two: Players) -> Game;
    #[storage(write)]fn insert_into_map(key: u64, value: u64);
    #[storage(read)]fn get_from_map(key: u64)-> u64;
}

impl TicTacToe for Contract {

    #[storage(write)]
    fn new_game(player_one: Players, player_two: Players) -> Game {
        let mut game = Game {
            PlayerOne: player_one,
            PlayerTwo: player_two,
            winner: Winners::None,
            playerTurn: player_one,
            //grid: [0,0,0,0,0,0,0,0,0],
        };
        storage.game = game;
        return game;
    }
    #[storage(write)]fn insert_into_map(key: u64, value: u64) {
        storage.map.insert(key, value);
    }

    #[storage(read)]fn get_from_map(key: u64) -> u64{
        return storage.map.get(key);
    }
}
