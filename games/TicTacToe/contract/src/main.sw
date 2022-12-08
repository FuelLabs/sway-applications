contract;

dep data_structures/game;
dep interface;

use game::Game;
use interface::TicTacToe;
use std::{auth::msg_sender, constants::ZERO_B256, hash::sha256, storage::StorageVec};

storage {
    games: Game = Game::default(),
    game_vec: StorageVec<(u64, Game)> = StorageVec {},
    game_boards: StorageMap<(u64, u64), Option<Identity>> = StorageMap {},
    board: StorageMap<(u64), Option<Identity>> = StorageMap {}, //To determine the winner StorageMap<(Game_id, Player_address),positions)>,
    board: u64 = ,

}

impl TicTacToe for Contract {
    #[storage(read, write)]
    fn new_game(player_one: Identity, player_two: Identity) -> Game {
        // This function initializes a new game by attributing each player to an address and the winner to None
        // The player_one is set to be the first to make a move
        // It also initializes a move counter that will increment its value by 1 each time a player makes a move
        let mut game = Game::new(player_one, player_two);
        let game_id = storage.game_vec.len();
        storage.game_vec.push((game_id, game));
        while game.move_counter < 9 {
            game.increment_move();
            storage.game_boards.insert((storage.game_vec.len(), game.move_counter), Option::None::<Identity>());
        }
        return game;
    }

    // This function first checks who's turn it is and then inserts to the storage map the player key + the position
    #[storage(read, write)]
    fn move(game_id: u64, position: u64) {
        // Return result instead of crashing
        let mut game = storage.game_vec.get(game_id).unwrap().1;

        require(game.winner.is_none(), "The game has already ended");
        require(position >= 1 && position <= 9, "Your move does not match any cell");
        
        let sender = msg_sender().unwrap();

        require(sender == game.players.player_one && game.player_turn == 1 || sender == game.players.player_two && game.player_turn == 2, "Invalid player");
        
        require(storage.game_boards.get((game_id, position)).is_none(), "Cell is not Empty");
        storage.game_boards.insert((game_id, position), Option::Some(sender));
        storage.player_positions.insert((game_id, position), Option::Some(sender));
    }

    #[storage(read)]
    fn grid_is_empty(game_id: u64) -> bool {
        // Check each cell. If one of them is empty (contains 0), then the map isn't full yet.
        // Acessing the state of each cell. If it's different than 0 (empty cell),
        // then it keeps counting the number of not-empty cells.
        // If all 9 cells are either 1 or 2, it means the map is full and the function returns true.
        // Else it's not full and the function returns false.
        let mut position = 0;
        let mut result = true;
        while position < 9 {
            position += 1;
            if storage.player_positions.get((game_id, position)).is_some() {
                result = false;
                break;
            }
        }

        result
    }

    // save the game and return the winner
    #[storage(write)]
    fn end_game(game_id: u64, game: Game) -> Option<Identity> {
        let current_game = game_boards.get(game_id, i).unwrap()
        if game_boards.get(game_id,0).unwrap() == game_boards.get(game_id,1) && if game_boards.get(game_id,0).unwrap() == if game_boards.get(game_id,2).unwrap() {

            }
        }

        storage.games = game;
        return game.winner;
    }


}