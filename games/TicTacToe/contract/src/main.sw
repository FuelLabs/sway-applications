contract;

dep data_structures;

use std::{
    address::Address,
    chain::auth::msg_sender,
    constants::ZERO_B256,
    hash::sha256,
    identity::Identity,
    option::Option,
    result::Result,
    revert::{require, revert},
    storage::StorageMap,
};

use data_structures::Game;

storage {
    game: Game = Game {
        PlayerOne: ~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000),
        PlayerTwo: ~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000),
        winner: Option::None,
    },
    player_turn: u64 = 0,
    map: StorageMap<(u64,
    u64), Option<Address>> = StorageMap {
    },
}

abi TicTacToe {
    #[storage(write)]fn new_game(player_one: Address, player_two: Address) -> Game;
    #[storage(read, write)]fn make_move(game: Game, position: u64);
    // #[storage(read, write)]fn next_player();
    #[storage(write)]fn end_game(game: Game) -> Option<Address>;
    #[storage(read)]fn map_is_full() -> bool;
}

impl TicTacToe for Contract {
    #[storage(write)]fn new_game(player_one: Address, player_two: Address) -> Game {
        let mut game = Game {
            PlayerOne: player_one,
            PlayerTwo: player_two,
            winner: Option::None,
        };
        storage.player_turn = 1;
        let mut counter = 0;
        while counter < 9 {
            counter += 1;
            storage.map.insert((1, counter), Option::None);
        }
        return game;
    }

    // This function first checks who's turn it is and then inserts to the storage map the player key + the position
    #[storage(read, write)]fn make_move(game: Game, position: u64) {
        let sender = msg_sender().unwrap();
        let address = match sender {
            Identity::Address(current_address) => {
                current_address
            },
            _ => {
                revert(42)
            },
        };
        require(game.winner.is_none(), "The game has already ended");
        require(position >= 1 && position <= 9, "Your move does not match any cell");
        if storage.player_turn == 1 {
            require(address == game.PlayerOne, "Not PlayerOne");
            storage.player_turn = 2;
        } else {
            require(address == game.PlayerTwo, "Not PlayerTwo");
            storage.player_turn = 1;
        }
        require(storage.map.get((1, position)).is_none(), "Cell is not Empty");
        storage.map.insert((1, position), Option::Some(address));
    }
}
//Check each cell. If one of them is empty (contains 0), then the map isn't full yet.
#[storage(read)]fn map_is_full() -> bool {
    // Acessing the state of each cell. If it's different than 0 (empty cell),
    // then it keeps counting the number of not-empty cells.
    // If all 9 cells are either 1 or 2, it means the map is full and the function returns true.
    // Else it's not full and the function returns false.
    let mut counter = 0;
    let mut result = true;
    while counter < 9 {
        counter += 1;
        if storage.map.get((1, counter)).is_none() {
            result = false;
            break;
        }
    }
    return result;
}

// save the game and return the winner
#[storage(write)]fn end_game(game: Game) -> Option<Address> {
    storage.game = game;
    return game.winner;
}
