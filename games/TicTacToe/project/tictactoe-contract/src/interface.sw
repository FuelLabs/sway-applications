library interface;

abi Game {
    #[storage(read, write)]
    fn new_game(player_one: Identity, player_two: Identity);

    #[storage(read, write)]
    fn make_move(position: u64);
}
