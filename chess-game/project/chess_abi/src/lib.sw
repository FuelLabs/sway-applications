library chess_abi;

use std::b512::B512;
use lib_chess::{game::Game, move::Move, events::*,};

abi Chess {
    /// Create a new game between 2 players. Can be called by anyone.
    /// bond option allows free play or pay to play.
    /// allow bond to be paid with this call. if bond sent is 2x required bond, both players bond is covered. if bond is exactly the bond amount, msg_sender's bond is covered.
    /// maybe have an optional config as well, i.e: disable 50 move rule, etc...
    #[storage(read, write)]
    fn start_new_game(player1: Identity, player2: Identity, bond: Option<u64>) -> b256;

    // #[storage(write)]
    // fn post_bond(game_id: b256);

    // #[storage(read)]
    // fn move(move: Move);

    // #[storage(read, write)]
    // fn move_from_state(nonce: u64, sig: B512);

    // #[storage(read)]
    // fn game(game_id: b256) -> Game;

    // fn game_id(player1: Identity, player2: Identity, nonce: u64) -> b256;

    // #[storage(read, write)]
    // fn claim_winnings(game_id: b256);

}
