library bitboard;

dep bitmaps;
use bitmaps::*;

/// The BitBoard type can be thought of as a stack of layers
/// which can be selectively combined to query the location of pieces.
pub struct BitBoard {
    black_pawns: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_knights: u64,
    black_queen: u64,
    black_king: u64,
    white_pawns: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_knights: u64,
    white_queen: u64,
    white_king: u64,
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
    black: u64,
    white: u64,
    all: u64,
}

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard {
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
            pawns: BLACK_PAWNS
            | WHITE_PAWNS,
            knights: BLACK_KNIGHTS
            | WHITE_KNIGHTS,
            bishops: BLACK_BISHOPS
            | WHITE_BISHOPS,
            rooks: BLACK_ROOKS
            | WHITE_ROOKS,
            queens: BLACK_QUEEN
            | WHITE_QUEEN,
            kings: BLACK_KING
            | WHITE_KING,
            black: BLACK_PIECES,
            white: WHITE_PIECES,
            all: ALL_PIECES,
        }
    }
}

//////////////////////////////////////////////////////////////////
/// TESTS
//////////////////////////////////////////////////////////////////
#[test()]
fn test_new_bitstack() {
    let board = BitBoard::new();
    assert(board.all == ALL_PIECES);
    // assert(board.pawns == BLACK_PAWNS | WHITE_PAWNS);
    // assert(board.knights == BLACK_KNIGHTS | WHITE_KNIGHTS);
    // assert(board.bishops == BLACK_BISHOPS | WHITE_BISHOPS);
    // assert(board.rooks == BLACK_ROOKS | WHITE_ROOKS);
    // assert(board.queens == BLACK_QUEEN | WHITE_QUEEN);
    // assert(board.kings == BLACK_KING | WHITE_KING);
    // assert(board.black == BLACK_PIECES);
    // assert(board.white == WHITE_PIECES);
}
