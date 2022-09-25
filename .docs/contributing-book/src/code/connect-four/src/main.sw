contract;

dep data_structures;
dep interface;

use data_structures::{Game, Player};
use interface::ConnectFour;

impl ConnectFour for Contract {
    fn create_game(player_two: Player, player_one: Player) -> Game {
        ~Game::new(player_one, player_two)
    }

    fn move(column: u64, game: Game) -> Game {
        game
    }
}
