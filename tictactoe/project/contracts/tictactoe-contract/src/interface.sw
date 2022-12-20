library interface;

dep data_structures;

use data_structures::GameState;

abi TicTacToe {
    #[storage(write)]
    fn new_game(player_one: Identity, player_two: Identity);

    #[storage(read, write)]
    fn play_move(position: u64);
}
