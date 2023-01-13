library bitstack;

dep bitmap;
use std::logging::log;
use bitmap::*;

/// The BitBoard type can be thought of as a stack of layers
/// which can be selectively combined to query the location of pieces.
pub struct BitStack {
    black_pawns: BitMap,
    black_bishops: BitMap,
    black_rooks: BitMap,
    black_knights: BitMap,
    black_queen: BitMap,
    black_king: BitMap,
    white_pawns: BitMap,
    white_bishops: BitMap,
    white_rooks: BitMap,
    white_knights: BitMap,
    white_queen: BitMap,
    white_king: BitMap,
    pawns: BitMap,
    knights: BitMap,
    bishops: BitMap,
    rooks: BitMap,
    queens: BitMap,
    kings: BitMap,
    black: BitMap,
    white: BitMap,
    all: BitMap,
    // pinned: BitMap,   //     REVIEW !
    // checkers: BitMap, //     REVIEW !
}

impl BitStack {
    pub fn new() -> BitStack {
        BitStack {
            black_pawns: BLACK_PAWNS,
            black_bishops: BLACK_BISHOPS,
            black_rooks: BLACK_ROOKS,
            black_knights: BLACK_KNIGHTS,
            black_queen: BLACK_QUEEN,
            black_king: BLACK_KING,
            white_pawns: WHITE_PAWNS,
            white_bishops: WHITE_BISHOPS,
            white_rooks: WHITE_ROOKS,
            white_knights: WHITE_KNIGHTS,
            white_queen: WHITE_QUEEN,
            white_king: WHITE_KING,
            all: ALL_PIECES,
            pawns: BLACK_PAWNS | WHITE_PAWNS,
            knights: BLACK_KNIGHTS | WHITE_KNIGHTS,
            bishops: BLACK_BISHOPS | WHITE_BISHOPS,
            rooks: BLACK_ROOKS | WHITE_ROOKS,
            queens: BLACK_QUEEN | WHITE_QUEEN,
            kings: BLACK_KING | WHITE_KING,
            black: BLACK_PIECES,
            white: WHITE_PIECES,
            // pinned: BitMap::new(),
            // checkers: BitMap::new(),
        }
    }
}

//////////////////////////////////////////////////////////////////
/// TESTS
//////////////////////////////////////////////////////////////////
#[test()]
fn test_new_bitstack() {
    let stack = BitStack::new();
    assert(stack.all == ALL_PIECES);
    // assert(stack.pawns == BLACK_PAWNS | WHITE_PAWNS);
    // assert(stack.knights == BLACK_KNIGHTS | WHITE_KNIGHTS);
    // assert(stack.bishops == BLACK_BISHOPS | WHITE_BISHOPS);
    // assert(stack.rooks == BLACK_ROOKS | WHITE_ROOKS);
    // assert(stack.queens == BLACK_QUEEN | WHITE_QUEEN);
    // assert(stack.kings == BLACK_KING | WHITE_KING);
    // assert(stack.black == BLACK_PIECES);
    // assert(stack.white == WHITE_PIECES);
    // assert(stack.pinned == 0x0000000000000000);
    // assert(stack.checkers == 0x0000000000000000);
}
