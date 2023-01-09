library lib_chess;

use std::constants::ZERO_B256;

pub const NUM_PIECE_TYPES: u8 = 6;
pub const NUM_COLORS: u8 = 2;
pub const BLACK: u8 = 0u8;
pub const WHITE: u8 = 1u8;

/**
Square Numbering

56 57 58 59 60 61 62 63
48 49 50 51 52 53 54 55
40 41 42 43 44 45 46 47
32 33 34 35 36 37 38 39
24 25 26 27 28 29 30 31
16 17 18 19 20 21 22 23
08 09 10 11 12 13 14 15
00 01 02 03 04 05 06 07
*/

/**
4 bits to represent piece on each square * 64 squares = 256 bits to store all pieces.
First bit denotes the colour: Black == 0, White == 1.
Remaining 3 bits specify the piece type.
*/

pub const EMPTY: u8 = 0u8;   // 000
pub const PAWN: u8 = 1u8;   // 001
pub const BISHOP: u8 = 2u8; // 010
pub const KNIGHT: u8 = 3u8; // 011
pub const ROOK: u8 = 4u8;   // 100
pub const QUEEN: u8 = 5u8;  // 101
pub const KING: u8 = 6u8;   // 110


// 4 bits to represent piece on each square, * 64 squares = 256bits to store all pieces.

/**
Inital binary board state:

0011 0100 0010 0101 0110 0010 0100 0011
0001 0001 0001 0001 0001 0001 0001 0001
0000 0000 0000 0000 0000 0000 0000 0000
0000 0000 0000 0000 0000 0000 0000 0000
0000 0000 0000 0000 0000 0000 0000 0000
0000 0000 0000 0000 0000 0000 0000 0000
1001 1001 1001 1001 1001 1001 1001 1001
1011 1100 1010 1101 1110 1010 1100 1011
*/

// the HEX equivalent of the above binary board state
const INITIAL_BOARD_STATE: b256 = 0x34256243111111110000000000000000000000000000000099999999BCADEACB;


pub struct BitBoard {
    bits: u64,
}

impl BitBoard {
    fn from_u64(value: u64) -> BitBoard {
        BitBoard {
            bits: value
        }
    }
}

// Primary Bitmaps
pub const WHITE_PAWNS: u64 = 0x000000000000FF00;
pub const WHITE_ROOKS: u64 = 0x0000000000000081;
pub const WHITE_KNIGHTS: u64 = 0x0000000000000042;
pub const WHITE_BISHOPS: u64 = 0x0000000000000024;
pub const WHITE_QUEEN: u64 = 0x0000000000000008;
pub const WHITE_KING: u64 = 0x0000000000000010;

pub const BLACK_PAWNS: u64 = 0x00FF000000000000;
pub const BLACK_ROOKS: u64 = 0x8100000000000000;
pub const BLACK_KNIGHTS: u64 = 0x4200000000000000;
pub const BLACK_BISHOPS: u64 = 0x2400000000000000;
pub const BLACK_QUEEN: u64 = 0x0800000000000000;
pub const BLACK_KING: u64 = 0x1000000000000000;

// BitBoards
pub const WHITE_PIECES: BitBoard = BitBoard::from_u64(0x000000000000FFFF);
pub const BLACK_PIECES: BitBoard = BitBoard::from_u64(0xFFFF000000000000);
pub const ALL_PIECES: BitBoard = BitBoard::from_u64(0xFFFF00000000FFFF);


// Utility Bitmaps
pub const BLANK: BitBoard = BitBoard::from_u64(0x0);
pub const RANK_1: u64 = 0x00000000000000FF;
pub const RANK_2: u64 = WHITE_PAWNS;
pub const RANK_3: u64 = 0x0000000000FF0000;
pub const RANK_4: u64 = 0x00000000FF000000;
pub const RANK_5: u64 = 0x000000FF00000000;
pub const RANK_6: u64 = 0x0000FF0000000000;
pub const RANK_7: u64 = BLACK_PAWNS;
pub const RANK_8: u64 = 0xFF00000000000000;

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = 0x0202020202020202;
pub const FILE_C: u64 = 0x0404040404040404;
pub const FILE_D: u64 = 0x0808080808080808;
pub const FILE_E: u64 = 0x1010101010101010;
pub const FILE_F: u64 = 0x2020202020202020;
pub const FILE_G: u64 = 0x4040404040404040;
pub const FILE_H: u64 = 0x8080808080808080;

pub const CASTLING_SQUARES_W_K: u64 = 0x0000000000000060;
pub const CASTLING_SQUARES_W_Q: u64 = 0x0000000000000006;
pub const CASTLING_SQUARES_B_K: u64 = 0x6000000000000000;
pub const CASTLING_SQUARES_B_Q: u64 = 0x0600000000000000;
pub const EDGES: u64 = 0xFF818181818181FF;
// pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AA;
// pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;
// pub const A1_H8_DIAGONAL: u64 = 0x8040201008040201;
// pub const H1_A8_ANTIDIAGONAL: u64 = 0x0102040810204080;


pub struct Game {
    player_1: Address,
    player_2: Address,
    board: Board,
    // nonce ?
}


pub enum BoardStatus {
    Active: (),
    Stalemate: (),
    Checkmate: (),
}


pub enum CastleRights {
    NoRights: (),
    KingSide: (),
    QueenSide: (),
    Both: (),
}

pub struct Board {
    pieces: [BitBoard; 6], 
    color_combined: [BitBoard; 2],
    combined: BitBoard,
    side_to_move: Color,
    castle_rights: [CastleRights; 2],
    pinned: BitBoard,
    checkers: BitBoard,
    hash: b256,
    en_passant: Option<Square>,
}

impl Board {
    fn new(bitboard: Option<BitBoard>) -> Board {
        let mut initial_bitboard = match bitboard {
            Option::Some(bb) => bb,
            Option::None(e) => BLANK, 
        };

        Board {
            pieces: [initial_bitboard; 6],
            color_combined: [BLANK; 2],
            combined: BLANK,
            side_to_move: Color::White,
            castle_rights: [CastleRights::NoRights; 2],
            pinned: BLANK,
            checkers: BLANK,
            hash: ZERO_B256,
            en_passant: Option::None,
        }
    }
}

pub struct Square {
    index: u8,
}

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

pub struct Move {
    source: Square,
    dest: Square,
    promotion: Option<Piece>,
}


pub enum Color {
    White: (),
    Black: (),
}

impl Color {
    fn to_index(self) -> u8 {
        match self {
            Color::Black => BLACK,
            Color::White => WHITE,
        }
    }
}

// impl Not for Color {
//     /// Get the other color.
//     fn not(self) -> Color {
//         if self == Color::White {
//             Color::Black
//         } else {
//             Color::White
//         }
//     }
// }

//////////////////////////////////////////////////////////////////
/// TESTS    
//////////////////////////////////////////////////////////////////

#[test()]
fn test_new_board() {
    let board = Board::new(Option::None);
            
}

