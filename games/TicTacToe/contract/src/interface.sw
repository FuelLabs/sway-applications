library interface;

dep data_structures/game;

use game::Game;

abi TicTacToe {
    #[storage(read, write)]
    fn new_game(player_one: Identity, player_two: Identity) -> Game;
    #[storage(read, write)]
    fn move(game_id: u64, position: u64);
    #[storage(write)]
    fn end_game(game: Game) -> Option<Identity>;
    #[storage(read)]
    fn grid_is_empty(game_id: u64) -> bool;
}
