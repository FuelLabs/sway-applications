library piece;

pub const NUM_PIECE_TYPES: u8 = 6u8;
pub const NUM_COLOURS: u8 = 2u8;
// TODO: consider using bools true & false to represent color
pub const BLACK: u8 = 0u8; // false
pub const WHITE: u8 = 1u8; // true

/**
4 bits to represent piece on each square:
First bit denotes the colour: Black == 0, White == 1.
Remaining 3 bits specify the piece type.
*/
pub const EMPTY: u8 = 0u8;   // 000
pub const PAWN: u8 = 1u8;   // 001
pub const BISHOP: u8 = 2u8; // 010
pub const ROOK: u8 = 3u8;   // 011
pub const KNIGHT: u8 = 4u8; // 100
pub const QUEEN: u8 = 5u8;  // 101
pub const KING: u8 = 6u8;   // 110

/**
Initial binary board state:

0011 0100 0010 0101 0110 0010 0100 0011
0001 0001 0001 0001 0001 0001 0001 0001
0000 0000 0000 0000 0000 0000 0000 0000
0000 0000 0000 0000 0000 0000 0000 0000
0000 0000 0000 0000 0000 0000 0000 0000
0000 0000 0000 0000 0000 0000 0000 0000
1001 1001 1001 1001 1001 1001 1001 1001
1011 1100 1010 1101 1110 1010 1100 1011

4 bits per piece * 64 squares = 256 bits to store all pieces.
HEX equivalent of the above binary board state:
*/
const INITIAL_BOARD_STATE: b256 = 0x34256243111111110000000000000000000000000000000099999999BCADEACB;

pub enum Piece {
    Pawn: (),
    Knight: (),
    Bishop: (),
    Rook: (),
    Queen: (),
    King: (),
}

impl Piece {
    fn to_u8(self) -> u8 {
        match self {
            Piece::Pawn => PAWN,
            Piece::Knight => KNIGHT,
            Piece::Bishop => BISHOP,
            Piece::Rook => ROOK,
            Piece::Queen => QUEEN,
            Piece::King => KING,
        }
    }
}

//////////////////////////////////////////////////////////////////
/// TESTS
//////////////////////////////////////////////////////////////////
#[test()]
fn test_to_u8() {
    let p1 = Piece::Pawn;
    let p2 = Piece::Bishop;
    let p3 = Piece::Rook;
    let p4 = Piece::Knight;
    let p5 = Piece::Queen;
    let p6 = Piece::King;

    assert(p1.to_u8() == 1u8);
    assert(p2.to_u8() == 2u8);
    assert(p3.to_u8() == 3u8);
    assert(p4.to_u8() == 4u8);
    assert(p5.to_u8() == 5u8);
    assert(p6.to_u8() == 6u8);
}