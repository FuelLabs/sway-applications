library move;

dep square;
dep piece;

use square::Square;
use piece::Piece;

// represents a move internally, but also represents the "message" format signed
// by players when making moves offchain.
pub struct Move {
    target: ContractId, // replay prevention
    nonce: u64,         // replay prevention
    game_id: b256,      // hash(player1_address, player2_address, game_number)
    piecemap: b256,
    metadata: u64,
    source: Square,
    dest: Square,
    promotion: Option<Piece>,
    sequence: u64       // seq must always be >= to stored seq for this game: https://programtheblockchain.com/posts/2018/05/11/state-channels-for-two-player-games/
}

impl Move {
    pub fn build(target: ContractId, nonce: u64, game_id: b256, piecemap: b256, metadata: u64, src: Square, dest: Square, promotion: Option<Piece>, seq: u64) -> Move {
        Move {
            target: target,
            nonce: nonce,
            game_id: game_id,
            piecemap: piecemap,
            metadata: metadata,
            source: src,
            dest: dest,
            promotion: promotion,
            sequence: seq,
        }
    }
}

// #[test()]
// fn test_move_builder() {
//     let sq_1 = Square::a2;
//     let sq_2 = Square::a3;
//     let my_move = Move::build(sq_1, sq_2, Option::None);
//     assert(my_move.source.to_index() == sq_1.to_index());
//     assert(my_move.dest.to_index() == sq_2.to_index());
//     let promo = if let Option::None = my_move.promotion {
//         false
//     } else {
//         true
//     };
//     assert(promo == false);
// }
