library game;

dep board;
dep game;

use std::address::Address;
use board::Board;
use game::Status;

pub struct Game {
    player_1: Address,
    player_2: Address,
    board: Board,
    status: Status,
    // last known state: hash(piecemap, metadata)
    // stored after a valid Move is applied to previous state.
    // piecemap & metadata are updated, stored along with bitstack, and logged.
    // TODO: determine if metadata.full_move_counter already gives us this property, i.e: check the move counter on the proposed move is the stored counter + 1 ?
    statehash: b256,
    // used to prevent signed message replays
    nonce: u64, // https://programtheblockchain.com/posts/2018/05/11/state-channels-for-two-player-games/
}

// TODO: add methods to conver to & from a status code, i.e:
// 0, 1, 2, 3
pub enum Status {
    Active: (),
    Stalemate: (),
    Checkmate: (),
}


//////////////////////////////////////////////////////////////////
/// TESTS
//////////////////////////////////////////////////////////////////
// #[test()]
// fn test_new_game() {
//     let game = game::new();

//     assert(game.player_1 == );
//     assert(game.player_2 == );
//     assert(game.game_id.unwrap().is_none());
//     assert(game.board.position == Position::new());
//     assert(game.board.bitboard == BitStack::new());

//     if let Status::Ready = game.status {
//         assert(true);
//     } else {
//         assert(false)
//     };
// }
