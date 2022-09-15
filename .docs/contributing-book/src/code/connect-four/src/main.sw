contract;

dep data_structures;
dep interface;
dep utils;

use data_structures::{Game, Player, Winner};
use interface::{ConnectFour, DrawEvent, MoveEvent, WinnerEvent};
use utils::validate_move;

storage {
    /// The total number of created games
    games_played: u64 = 0,
    /// The number of times player 2 has won against player 1
    player_two_wins: u64 = 0,
    /// The number of times player 1 has won against player 2
    player_one_wins: u64 = 0,
   
   // ...
}

impl ConnectFour for Contract {
    fn create_game(player_two: Player, player_one: Player) -> Game {
      // Perform a check on each player address to see if they are blacklisted
      // owl
        ~Game::new(player_one, player_two)
    }

    fn move(column: u64, game: Game) -> Game {
      // Perform a check to see if the game has ended
      // Perform a check to see if the position is valid
        validate_move(column, game);

      
      // owl
        game
    }

   // rest of owl
}
