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
    revert::{
        require,
        revert,
    },
    storage::{
        StorageMap,
        StorageVec,
    },
};

use data_structures::Game;
storage {
    games: Game = Game {
        PlayerOne: ~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000),
        PlayerTwo: ~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000),
        winner: Option::None,
        player_turn: 1,
        move_counter: 0,
    },
    game_vec: StorageVec<Game> = StorageVec {},
    game_boards: StorageMap<(u64, u64), Option<Address>> = StorageMap {},
    player_positions: StorageMap<(u64, Address), Vec<u64>> = StorageMap {},
}

abi TicTacToe {
    #[storage(read, write)]
    fn new_game(player_one: Address, player_two: Address) -> Game;
    #[storage(read, write)]
    fn make_move(game_id: u64, mut game: Game, position: u64);
    #[storage(write)]
    fn end_game(game: Game) -> Option<Address>;
    #[storage(read)]
    fn map_is_full(game_id: u64) -> bool;
}

impl TicTacToe for Contract {
    #[storage(read, write)]
    fn new_game(player_one: Address, player_two: Address) -> Game {
        // This function initializes a new game by attributing each player to an address and the winner to None
        // The player_one is set to be the first to make a move
        // It also initializes a move counter that will increment its value by 1 each time a player makes a move
        let mut game = Game {
            PlayerOne: player_one,
            PlayerTwo: player_two,
            winner: Option::None,
            player_turn: 1,
            move_counter: 0,
        };
        storage.game_vec.push(game);
        while game.move_counter < 9 {
            game.move_counter += 1;
            storage.game_boards.insert((storage.game_vec.len(), game.move_counter), Option::None);
        }
        return game;
    }

    // This function first checks who's turn it is and then inserts to the storage map the player key + the position
    #[storage(read, write)]
    fn make_move(game_id: u64, mut game: Game, position: u64) {
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
        if game.player_turn == 1 {
            require(address == game.PlayerOne, "Not PlayerOne");
            game.player_turn = 2;
        } else {
            require(address == game.PlayerTwo, "Not PlayerTwo");
            game.player_turn = 1;
        }
        require(storage.game_boards.get((game_id, position)).is_none(), "Cell is not Empty");
        storage.game_boards.insert((game_id, position), Option::Some(address));
    }

    //Check each cell. If one of them is empty (contains 0), then the map isn't full yet.
    #[storage(read)]
    fn map_is_full(game_id: u64) -> bool {
        // Acessing the state of each cell. If it's different than 0 (empty cell),
        // then it keeps counting the number of not-empty cells.
        // If all 9 cells are either 1 or 2, it means the map is full and the function returns true.
        // Else it's not full and the function returns false.
        let mut move_counter = 0;
        let mut result = true;
        while move_counter < 9 {
            move_counter += 1;
            if storage.game_boards.get((game_id, move_counter)).is_none()
            {
                result = false;
                break;
            }
        }
        return result;
    }

    // save the game and return the winner
    #[storage(write)]
    fn end_game(game: Game) -> Option<Address> {
        storage.games = game;
        return game.winner;
    }
}
