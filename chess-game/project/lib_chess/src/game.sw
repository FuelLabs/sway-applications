library game;

dep board;

use board::Board;

// TODO: add methods to conver to & from a status code, i.e:
// 0, 1, 2, 3
pub enum Status {
    Active: (),
    Stalemate: (),
    Checkmate: (),
}

pub struct Game {
    // could store players as an array or tuple. Game.players.WHITE, Game.players.BLACK ...
    player_1: Identity,
    player_2: Identity,
    match_number: u64, // tracks games played between P1 & P2.
    board: Board,
    status: Status,
    // should have a "winner" field... i.e: winner: Option<Identity>,
    statehash: b256,   // TODO: maybe move this to Board ?
}

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
