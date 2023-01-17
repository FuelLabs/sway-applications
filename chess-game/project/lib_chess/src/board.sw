library board;

dep bitboard;
dep errors;
dep move;
dep piece;
dep special;
dep square;
dep utils;

use bitboard::BitBoard;
use errors::*;
use move::Move;
use piece::{BISHOP, BLACK, EMPTY, KING, KNIGHT, PAWN, Piece, QUEEN, ROOK, WHITE};
use special::CastleRights;
use square::Square;
use utils::{b256_multimask, compose, decompose, multi_bit_mask, query_bit, toggle_bit, turn_on_bit};
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
pub const INITIAL_METADATA: u64 = 0b00000000_00000000_00000000_00000000_00001111_00000000_00000000_00000001;
pub const HALF_MOVE_MASK: u64 = 0x000000000000FF00;
pub const FULL_MOVE_MASK: u64 = 0x000000FF00000000;
pub const EN_PASSANT_MASK: u64 = 0x0000000000FF0000;
pub const CASTLING_MASK: u64 = 0x00000000FF000000;
pub const FULL_MOVE_MASK: u64 = 0x000000FF00000000;
pub const HALF_MOVE_CLEARING_MASK: u64 = 0xFFFFFFFFFFFF00FF;
pub const FULL_MOVE_CLEARING_MASK: u64 = 0xFFFFFF00FFFFFFFF;
pub const CASTLING_CLEARING_MASK: u64 = 0xFFFFFFFF00FFFFFF;
pub const EN_PASSANT_CLEARING_MASK: u64 = 0xFFFFFFFFFF00FFFF;

// struct for internal state representation.
// bitboards are calculated from the piecemap
pub struct Board {
    // complete location and type data for the board at a given point in time. Efficient transport, but not efficient to query, i.e: "give me all non-pinned white pawns", etc...
    piecemap: b256,
    // Great for answering queries, but less efficient for transport.
    // less efficient at answering the question: "what color/type is the piece on square f7?"
    bitboard: BitBoard,
    metadata: u64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            piecemap: INITIAL_PIECEMAP,
            bitboard: BitBoard::new(),
            metadata: INITIAL_METADATA,
        }
    }
}

impl Board {
    pub fn build(pieces: b256, bits: BitBoard, data: u64) -> Board {
        Board {
            piecemap: pieces,
            bitboard: bits,
            metadata: data,
        }
    }
}

impl Board {
    pub fn clear_castling_rights(mut self) {
        self.metadata = self.metadata & CASTLING_CLEARING_MASK;
    }

    pub fn clear_en_passant(mut self) {
        self.metadata = self.metadata & EN_PASSANT_CLEARING_MASK;
    }

     // clear 1 nibble corresponding to a specific square's index from a piecemap
    pub fn clear_square(mut self, square: Square) {
        let mut index = square.to_index();
        // create a mask of all 1's except 4 0's on the target nibble.
        if index == 0 {
            let first_nibble_mask = b256_multimask(252);
            self.piecemap = self.piecemap & first_nibble_mask;
        } else {
            // eg: index = 42, * 4 = 168th bit
            // part 1: need 256 - 168 - 4 `1`s, << 168 + 4 bits.
            // part 2: need 168 `1`s
            // mask = part 1 | part 2
            let nibble_index = index * 4;
            let mask_part_1 = b256_multimask((256 - (nibble_index) - 4) << nibble_index);
            let mask_part_2 = b256_multimask(nibble_index);
            self.piecemap = self.piecemap & (mask_part_1 | mask_part_2);
        }
    }
}

impl Board {
    pub fn write_square_to_piecemap(mut self, color: u64, piece: Piece, dest: Square) {
        self.clear_square(dest);
        let mut index = dest.to_index();
        // set the "color" bit in the piece code
        let colored_piece = piece.to_u64() | (color << 4);
        let mut piece_code = compose((0, 0, 0, (colored_piece)));
        let shifted = piece_code << index;
        self.piecemap = self.piecemap | shifted;
    }

    pub fn half_move_counter(self) -> u64 {
        (self.metadata & HALF_MOVE_MASK) >> 8
    }

    pub fn full_move_counter(self) -> u64 {
        (self.metadata & FULL_MOVE_MASK) >> 32
    }

    pub fn en_passant_target(self) -> Square {
        Square::from_index((self.metadata & EN_PASSANT_MASK) >> 16).unwrap()
    }

    // TODO: consider partial reads, i.e: read only black castling rights if it's Blacks turn to move.
    pub fn castling_rights(self) -> Result<[CastleRights; 2], ChessError> {
        let value = (self.metadata & CASTLING_MASK) >> 24;
        match value {
            0x0 => Result::Ok([CastleRights::NoRights, CastleRights::NoRights]),
            0x1 => Result::Ok([CastleRights::NoRights, CastleRights::KingSide]),
            0x2 => Result::Ok([CastleRights::NoRights, CastleRights::QueenSide]),
            0x3 => Result::Ok([CastleRights::NoRights, CastleRights::Both]),
            0x4 => Result::Ok([CastleRights::KingSide, CastleRights::NoRights]),
            0x5 => Result::Ok([CastleRights::KingSide, CastleRights::KingSide]),
            0x6 => Result::Ok([CastleRights::KingSide, CastleRights::QueenSide]),
            0x7 => Result::Ok([CastleRights::KingSide, CastleRights::Both]),
            0x8 => Result::Ok([CastleRights::QueenSide, CastleRights::NoRights]),
            0x9 => Result::Ok([CastleRights::QueenSide, CastleRights::KingSide]),
            0xA => Result::Ok([CastleRights::QueenSide, CastleRights::QueenSide]),
            0xB => Result::Ok([CastleRights::QueenSide, CastleRights::Both]),
            0xC => Result::Ok([CastleRights::Both, CastleRights::NoRights]),
            0xD => Result::Ok([CastleRights::Both, CastleRights::KingSide]),
            0xE => Result::Ok([CastleRights::Both, CastleRights::QueenSide]),
            0xF => Result::Ok([CastleRights::Both, CastleRights::Both]),
            _ => Result::Err(ChessError::Unimplemented),
        }
    }

    pub fn set_castling_rights(mut self, rights: (CastleRights, CastleRights)) {
        self.clear_castling_rights();
        let value = match rights {
            (CastleRights::NoRights, CastleRights::NoRights) => 0x0,
            (CastleRights::NoRights, CastleRights::KingSide) => 0x1,
            (CastleRights::NoRights, CastleRights::QueenSide) => 0x2,
            (CastleRights::NoRights, CastleRights::Both) => 0x3,
            (CastleRights::KingSide, CastleRights::NoRights) => 0x4,
            (CastleRights::KingSide, CastleRights::KingSide) => 0x5,
            (CastleRights::KingSide, CastleRights::QueenSide) => 0x6,
            (CastleRights::KingSide, CastleRights::Both) => 0x7,
            (CastleRights::QueenSide, CastleRights::NoRights) => 0x8,
            (CastleRights::QueenSide, CastleRights::KingSide) => 0x9,
            (CastleRights::QueenSide, CastleRights::QueenSide) => 0xA,
            (CastleRights::QueenSide, CastleRights::Both) => 0xB,
            (CastleRights::Both, CastleRights::NoRights) => 0xC,
            (CastleRights::Both, CastleRights::KingSide) => 0xD,
            (CastleRights::Both, CastleRights::QueenSide) => 0xE,
            (CastleRights::Both, CastleRights::Both) => 0xF,
        };

        self.metadata = self.metadata | (value << 24);
    }

    pub fn reset_half_move(mut self) {
        self.metadata = self.metadata & HALF_MOVE_CLEARING_MASK;
    }

    pub fn clear_full_move(mut self) {
        self.metadata = self.metadata & FULL_MOVE_CLEARING_MASK;
    }

    pub fn read_square(self, square_index: u64) -> (u64, Piece) {
        let mut index = square_index;
        let mut mask = compose((0, 0, 0, multi_bit_mask(4)));
        let piece_code = if index == 0 {
            decompose(self.piecemap & mask).3
        } else {
            index *= 4;
            let mask = compose((0, 0, 0, multi_bit_mask(index) << index));
            decompose((self.piecemap & mask) >> index).3
        };
        let color = piece_code >> 4;
        let piece = Piece::from_u64(piece_code).unwrap();
        (color, piece)
    }
}

impl Board {
    // convert bitboard to piecemap
    // TODO: do I ever need to perform all these steps, or can I always just use the latest Move to update 2 nibbles in the piecemap?
    pub fn generate_piecemap(mut self) {
        let mut i = 0;
        let mut mask = 0;
        let mut color = 0;
        let mut piece = EMPTY;
        // TODO: see if I can use match to clean this up. Add unit tests first so I know it actually works.
        while i < 64 {
            mask = 1 << i;
            let occupied = mask & self.bitboard.all;
            if occupied == 0 {
                i += 1;
            } else {
                let pawn = mask & self.bitboard.pawns;
                if pawn == 1 {
                    piece = PAWN;
                } else {
                    let bishop = mask & self.bitboard.bishops;
                    if bishop == 1 {
                        piece = BISHOP;
                    } else {
                        let rook = mask & self.bitboard.rooks;
                        if rook == 1 {
                            piece = ROOK;
                        } else {
                            let knight = mask & self.bitboard.knights;
                            if knight == 1 {
                                piece = KNIGHT;
                            } else {
                                let queen = mask & self.bitboard.queens;
                                if queen == 1 {
                                    piece = QUEEN;
                                } else {
                                    piece = KING;
                                }
                            }
                        }
                    };
                }
            };

            let color = if mask & self.bitboard.black == 0 {
                BLACK
            } else {
                WHITE
            };

            self.write_square_to_piecemap(color, Piece::from_u64(piece).unwrap(), Square::from_index(i).unwrap());
            i += 1;
        };
    }

    // wraps Square::clear() & Square::set() ??                  REVIEW !
    pub fn move_piece(mut self, src: Square, dest: Square) {
        let (color, piece) = self.read_square(src.to_index());
        // clear src
        self.clear_square(src);
        // TODO: clear dest if !color, and must be legal move
        self.clear_square(dest);
        // set src
        self.write_square_to_piecemap(color, piece, dest);
    }

    pub fn side_to_move(self) -> u64 {
        query_bit(self.metadata, 0)
    }

    pub fn toggle_side_to_move(mut self) {
        self.metadata = toggle_bit(self.metadata, 0);
    }

    pub fn increment_half_move_counter(mut self) {
        let value = self.half_move_counter();
        self.reset_half_move();
        self.metadata = self.metadata | ((value + 1) << 8);
    }

    pub fn increment_full_move_counter(mut self) {
        let value = self.full_move_counter();
        self.clear_full_move();
        self.metadata = self.metadata | ((value + 1) << 32);
    }

    pub fn set_en_passant(mut self, target: Square) {
        self.clear_en_passant();
        self.metadata = self.metadata | target.to_index() << 16;
    }
}

impl Board {
    // TODO: review this, inputs/outputs & mutation of self?
    pub fn write_to_bitboard(mut self, board: Board) {
        let mut bitboard = BitBoard::new();

        let mut s = 0;
        let mut i = 0;
        while i < 64 {
            let (color, piece) = board.read_square(s);
            if color == BLACK {
                match piece {
                    Piece::Pawn => self.bitboard.black_pawns = turn_on_bit(bitboard.black_pawns, i),
                    Piece::Bishop => self.bitboard.black_bishops = turn_on_bit(bitboard.black_bishops, i),
                    Piece::Rook => self.bitboard.black_rooks = turn_on_bit(bitboard.black_rooks, i),
                    Piece::Knight => self.bitboard.black_knights = turn_on_bit(bitboard.black_knights, i),
                    Piece::Queen => self.bitboard.black_queen = turn_on_bit(bitboard.black_queen, i),
                    Piece::King => self.bitboard.black_king = turn_on_bit(bitboard.black_king, i),
                }
            } else {
                match piece {
                    Piece::Pawn => self.bitboard.white_pawns = turn_on_bit(bitboard.white_pawns, i),
                    Piece::Bishop => self.bitboard.white_bishops = turn_on_bit(bitboard.white_bishops, i),
                    Piece::Rook => self.bitboard.white_rooks = turn_on_bit(bitboard.white_rooks, i),
                    Piece::Knight => self.bitboard.white_knights = turn_on_bit(bitboard.white_knights, i),
                    Piece::Queen => self.bitboard.white_queen = turn_on_bit(bitboard.white_queen, i),
                    Piece::King => self.bitboard.white_king = turn_on_bit(bitboard.white_king, i),
                }
            };
            s += 4;
            i += 1;
        }
    }

    // TODO: consider making this a method on Board
    // this method assumes that the Board and the Move have already been validated !
    // TODO: move all validation to validate_proposed_move()
    // TODO: rename to apply_transition()
    // transition should just apply the move and update data structures accordingly.
    pub fn transition(mut self, move: Move) {
        // update metadata:
        self.toggle_side_to_move();
        let turn = self.increment_half_move_counter();
        let half_move = self.half_move_counter();
        if half_move > 0 && half_move % 2 == 0 {
            self.increment_full_move_counter();
        };
        // update en_passant if needed
        if move.dest.to_index() == self.en_passant_target().to_index()
        {
            self.clear_en_passant();
        };

        /**
        let (allowed, maybe_square) = move.allows_en_passant();
        if allowed {
            self.set_en_passant(maybe_square.unwrap())
        }
        */
        /**
        // update castling_rights if needed
        if move.is_castling() {
            let mut rights = self.castling_rights();
            let whose_turn = self.side_to_move();
            match whose_turn {
                color::Black => {
                    self.set_castling_rights((CastleRights::NoRights, rights[1].unwrap()));
                },
                Color::White => {
                    self.set_castling_rights((rights[0].unwrap(), CastleRights::NoRights));
                },
            };
        }
        */
        // these are likely needed in validate_move()
        // let mut bitboard = self.generate_bitboard();
        // self.write_piecemap(bitboard);
        /**
        // read the piece on src square
        let piece = self.square(move.source);
        // set the piece on dest and clear src
        self.move_piece(move.src, move.dest, color, piece);
        */
    }
}

//////////////////////////////////////////////////////////////////
/// TESTS
//////////////////////////////////////////////////////////////////
#[test()]
fn test_new_board() {
    let board = Board::new();
    assert(board.piecemap == INITIAL_PIECEMAP);
    assert(board.metadata == INITIAL_METADATA);
}

// #[test()]
// fn test_transition_side_to_move() {
//     let mut p1 = Board::build(INITIAL_PIECEMAP, BitBoard::new(), INITIAL_METADATA);
//     let m1 = Move::build(Square::a3, Square::a4, Option::None);
//     p1.transition(m1);
//     assert(p1.side_to_move() == BLACK);
//     let m2 = Move::build(Square::a2, Square::a3, Option::None);
//     p1.transition(m2);
//     assert(p1.side_to_move() == WHITE);
// }
// #[test()]
// fn test_transition_half_move_increment() {
//     let mut p1 = Board::build(INITIAL_PIECEMAP, BitBoard::new(),INITIAL_METADATA);
//     let m1 = Move::build(Square::a2, Square::a3, Option::None);
//     p1.transition(m1);
//     assert(p1.half_move_counter() == 1);
// }
// #[test()]
// fn test_increment_full_move_counter() {
//     let metadata = 0b00000000_00000000_00000000_00000000_00001111_00000000_00000000_00000001;
//     let mut p1 = Board::build(INITIAL_PIECEMAP, BitBoard::new(),metadata);
//     let m1 = Move::build(Square::a2, Square::a3, Option::None);
//     p1.transition(m1);
//     assert(p1.half_move_counter() == 1);
//     assert(p1.full_move_counter() == 0);
//     p1.transition(m1);
//     assert(p1.half_move_counter() == 2);
//     assert(p1.full_move_counter() == 1);
//     p1.transition(m1);
//     assert(p1.half_move_counter() == 3);
//     assert(p1.full_move_counter() == 1);
//     p1.transition(m1);
//     assert(p1.half_move_counter() == 4);
//     assert(p1.full_move_counter() == 2);
// }
#[test()]
fn test_increment_half_move_counter() {
    let mut p1 = Board::new();
    assert(p1.half_move_counter() == 0);
    p1.increment_half_move_counter();
    assert(p1.half_move_counter() == 1);
}
