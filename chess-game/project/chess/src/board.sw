library board;

dep bitstack;

use bitstack::BitStack;

/**

note: for more detail about how pieces are encoded, see ./piece.sw

Initial board state:

    0011 0100 0010 0101 0110 0010 0100 0011
    0001 0001 0001 0001 0001 0001 0001 0001
    0000 0000 0000 0000 0000 0000 0000 0000
    0000 0000 0000 0000 0000 0000 0000 0000
    0000 0000 0000 0000 0000 0000 0000 0000
    0000 0000 0000 0000 0000 0000 0000 0000
    1001 1001 1001 1001 1001 1001 1001 1001
    1011 1100 1010 1101 1110 1010 1100 1011

4 bits per piece * 64 squares = 256 bits to store all pieces.
*/

// HEX equivalent of the above starting board state
pub const INITIAL_PIECEMAP: b256 = 0x34256243111111110000000000000000000000000000000099999999BCADEACB;




// struct for data transport
// replacement for FEN, unless can find way to encode all in single b256

// piecemap: b256
// metadata: u64,
// statehash: b256, ?



// struct for internal state representation.
// bitstacks are calculated from the piecemap
pub struct Board {
    piecemap: b256,
    bitboard: BitStack,
    metadata: u64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            piecemap: INITIAL_PIECEMAP,
            bitboard: BitStack::new(),
            metadata: 0x0000000000000000,
        }
    }
}

//////////////////////////////////////////////////////////////////
/// TESTS
//////////////////////////////////////////////////////////////////
#[test()]
fn test_new_board() {
    let board = Board::new();
    assert(board.piecemap == INITIAL_PIECEMAP);
    assert(board.metadata == 0x0000000000000000);
}