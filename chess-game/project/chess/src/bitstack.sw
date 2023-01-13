library bitstack;

dep bitmap;

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
    let board = BitStack::new();

    assert(board.all.bits == 0xFFFF00000000FFFF);
    assert(board.pawns.bits == 0x00FF000000000000 | 0x000000000000FF00);
    assert(board.knights.bits == 0x4200000000000000 | 0x0000000000000042);
    assert(board.bishops.bits == 0x2400000000000000 | 0x0000000000000024);
    assert(board.rooks.bits == 0x8100000000000000 | 0x0000000000000081);
    assert(board.queens.bits == 0x0800000000000000 | 0x0000000000000008);
    assert(board.kings.bits == 0x1000000000000000 | 0x0000000000000010);
    assert(board.black.bits == 0xFFFF000000000000);
    assert(board.white.bits == 0x000000000000FFFF);
    // assert(board.pinned.bits == 0x0000000000000000);
    // assert(board.checkers.bits == 0x0000000000000000);
}
