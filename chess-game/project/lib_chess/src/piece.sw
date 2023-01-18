library piece;

dep errors;

use errors::ChessError;


pub const NUM_PIECE_TYPES: u8 = 6u8;
pub const NUM_COLOURS: u8 = 2u8;
// TODO: consider using bools true & false to represent color
pub const BLACK: u8 = 0u8; // false
pub const WHITE: u8 = 1u8; // true

/**
4 bits to represent piece on each square:
First bit denotes the color: Black == 0, White == 1.
Remaining 3 bits specify the piece type.
*/
pub const EMPTY: u64 = 0;  // 000
pub const PAWN: u64 = 1;   // 001
pub const BISHOP: u64 = 2; // 010
pub const ROOK: u64 = 3;   // 011
pub const KNIGHT: u64 = 4; // 100
pub const QUEEN: u64 = 5;  // 101
pub const KING: u64 = 6;   // 110

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
    Bishop: (),
    Rook: (),
    Knight: (),
    Queen: (),
    King: (),
}

impl Piece {
    pub fn to_u64(self) -> u64 {
        match self {
            Piece::Pawn => PAWN,
            Piece::Bishop => BISHOP,
            Piece::Rook => ROOK,
            Piece::Knight => KNIGHT,
            Piece::Queen => QUEEN,
            Piece::King => KING,
        }
    }

    pub fn from_u64(piece_code: u64) -> Result<Piece, ChessError> {
        let piece_color_mask = 0b0111;
        let colorless_piece = piece_code & piece_color_mask;
        match colorless_piece {
            PAWN => Result::Ok(Piece::Pawn),
            KNIGHT => Result::Ok(Piece::Knight),
            BISHOP => Result::Ok(Piece::Bishop),
            ROOK => Result::Ok(Piece::Rook),
            QUEEN => Result::Ok(Piece::Queen),
            KING => Result::Ok(Piece::King),
            _ => Result::Err(ChessError::Unimplemented),
        }
    }
}

//////////////////////////////////////////////////////////////////
/// TESTS
//////////////////////////////////////////////////////////////////
#[test()]
fn test_piece_to_u8() {
    let p1 = Piece::Pawn;
    let p2 = Piece::Bishop;
    let p3 = Piece::Rook;
    let p4 = Piece::Knight;
    let p5 = Piece::Queen;
    let p6 = Piece::King;

    assert(p1.to_u64() == 1);
    assert(p2.to_u64() == 2);
    assert(p3.to_u64() == 3);
    assert(p4.to_u64() == 4);
    assert(p5.to_u64() == 5);
    assert(p6.to_u64() == 6);
}