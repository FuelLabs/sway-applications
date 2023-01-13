library square;

dep errors;
use errors::ChessError;
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

pub enum Square {
    a1: (), b1: (), c1: (), d1: (), e1: (), f1: (), g1: (), h1: (),
    a2: (), b2: (), c2: (), d2: (), e2: (), f2: (), g2: (), h2: (),
    a3: (), b3: (), c3: (), d3: (), e3: (), f3: (), g3: (), h3: (),
    a4: (), b4: (), c4: (), d4: (), e4: (), f4: (), g4: (), h4: (),
    a5: (), b5: (), c5: (), d5: (), e5: (), f5: (), g5: (), h5: (),
    a6: (), b6: (), c6: (), d6: (), e6: (), f6: (), g6: (), h6: (),
    a7: (), b7: (), c7: (), d7: (), e7: (), f7: (), g7: (), h7: (),
    a8: (), b8: (), c8: (), d8: (), e8: (), f8: (), g8: (), h8: (),
}

impl Square {
    pub fn to_index(self) -> u64 {
        match self {
            Square::a1 => 0, Square::b1 => 1, Square::c1 => 2, Square::d1 => 3, Square::e1 => 4, Square::f1 => 5, Square::g1 => 6, Square::h1 => 7,
            Square::a2 => 8, Square::b2 => 9, Square::c2 => 10, Square::d2 => 11, Square::e2 => 12, Square::f2 => 13, Square::g2 => 14, Square::h2 => 15,
            Square::a3 => 16, Square::b3 => 17, Square::c3 => 18, Square::d3 => 19, Square::e3 => 20, Square::f3 => 21, Square::g3 => 22, Square::h3 => 23,
            Square::a4 => 24, Square::b4 => 25, Square::c4 => 26, Square::d4 => 27, Square::e4 => 28, Square::f4 => 29, Square::g4 => 30, Square::h4 => 31,
            Square::a5 => 32, Square::b5 => 33, Square::c5 => 34, Square::d5 => 35, Square::e5 => 36, Square::f5 => 37, Square::g5 => 38, Square::h5 => 39,
            Square::a6 => 40, Square::b6 => 41, Square::c6 => 42, Square::d6 => 43, Square::e6 => 44, Square::f6 => 45, Square::g6 => 46, Square::h6 => 47,
            Square::a7 => 48, Square::b7 => 49, Square::c7 => 50, Square::d7 => 51, Square::e7 => 52, Square::f7 => 53, Square::g7 => 54, Square::h7 => 55,
            Square::a8 => 56, Square::b8 => 57, Square::c8 => 58, Square::d8 => 59, Square::e8 => 60, Square::f8 => 61, Square::g8 => 62, Square::h8 => 63,
        }
    }

    pub fn from_index(index: u64) -> Result<Square, ChessError> {
        match index {
            0 => Result::Ok(Square::a1), 1 => Result::Ok(Square::b1), 2 => Result::Ok(Square::c1), 3 => Result::Ok(Square::d1), 4 => Result::Ok(Square::e1), 5 => Result::Ok(Square::f1), 6 => Result::Ok(Square::g1), 7 => Result::Ok(Square::h1),
            8 => Result::Ok(Square::a2), 9 => Result::Ok(Square::b2), 10 => Result::Ok(Square::c2), 11 => Result::Ok(Square::d2), 12 => Result::Ok(Square::e2), 13 => Result::Ok(Square::f2), 14 => Result::Ok(Square::g2), 15 => Result::Ok(Square::h2),
            16 => Result::Ok(Square::a3), 17 => Result::Ok(Square::b3), 18 => Result::Ok(Square::c3), 19 => Result::Ok(Square::d3), 20 => Result::Ok(Square::e3), 21 => Result::Ok(Square::f3), 22 => Result::Ok(Square::g3), 23 => Result::Ok(Square::h3),
            24 => Result::Ok(Square::a4), 25 => Result::Ok(Square::b4), 26 => Result::Ok(Square::c4), 27 => Result::Ok(Square::d4), 28 => Result::Ok(Square::e4), 29 => Result::Ok(Square::f4), 30 => Result::Ok(Square::g4), 31 => Result::Ok(Square::h4),
            32 => Result::Ok(Square::a5), 33 => Result::Ok(Square::b5), 34 => Result::Ok(Square::c5), 35 => Result::Ok(Square::d5), 36 => Result::Ok(Square::e5), 37 => Result::Ok(Square::f5), 38 => Result::Ok(Square::g5), 39 => Result::Ok(Square::h5),
            40 => Result::Ok(Square::a6), 41 => Result::Ok(Square::b6), 42 => Result::Ok(Square::c6), 43 => Result::Ok(Square::d6), 44 => Result::Ok(Square::e6), 45 => Result::Ok(Square::f6), 46 => Result::Ok(Square::g6), 47 => Result::Ok(Square::h6),
            48 => Result::Ok(Square::a7), 49 => Result::Ok(Square::b7), 50 => Result::Ok(Square::c7), 51 => Result::Ok(Square::d7), 52 => Result::Ok(Square::e7), 53 => Result::Ok(Square::f7), 54 => Result::Ok(Square::g7), 55 => Result::Ok(Square::h7),
            56 => Result::Ok(Square::a8), 57 => Result::Ok(Square::b8), 58 => Result::Ok(Square::c8), 59 => Result::Ok(Square::d8), 60 => Result::Ok(Square::e8), 61 => Result::Ok(Square::f8), 62 => Result::Ok(Square::g8), 63 => Result::Ok(Square::h8), _ => Result::Err(ChessError::OutOfBounds),
        }
    }
}

// impl core::ops::Eq for Square {
//     fn eq(self, other: Self) -> bool {
//         match (self, other) {
//             (Square::a1, Square::a1) => true,
//             (Square::a2, Square::a2) => true,
//             (Square::a3, Square::a3) => true,
//             (Square::a4, Square::a4) => true,
//             (Square::a5, Square::a5) => true,
//             (Square::a6, Square::a6) => true,
//             (Square::a7, Square::a7) => true,
//             (Square::a8, Square::a8) => true,
//             //
//             (Square::b1, Square::b1) => true,
//             (Square::b2, Square::b2) => true,
//             (Square::b3, Square::b3) => true,
//             (Square::b4, Square::b4) => true,
//             (Square::b5, Square::b5) => true,
//             (Square::b6, Square::b6) => true,
//             (Square::b7, Square::b7) => true,
//             (Square::b8, Square::b8) => true,
//             //
//             (Square::c1, Square::c1) => true,
//             (Square::c2, Square::c2) => true,
//             (Square::c3, Square::c3) => true,
//             (Square::c4, Square::c4) => true,
//             (Square::c5, Square::c5) => true,
//             (Square::c6, Square::c6) => true,
//             (Square::c7, Square::c7) => true,
//             (Square::c8, Square::c8) => true,
//             //
//             (Square::d1, Square::d1) => true,
//             (Square::d2, Square::d2) => true,
//             (Square::d3, Square::d3) => true,
//             (Square::d4, Square::d4) => true,
//             (Square::d5, Square::d5) => true,
//             (Square::d6, Square::d6) => true,
//             (Square::d7, Square::d7) => true,
//             (Square::d8, Square::d8) => true,
//             //
//             (Square::e1, Square::e1) => true,
//             (Square::e2, Square::e2) => true,
//             (Square::e3, Square::e3) => true,
//             (Square::e4, Square::e4) => true,
//             (Square::e5, Square::e5) => true,
//             (Square::e6, Square::e6) => true,
//             (Square::e7, Square::e7) => true,
//             (Square::e8, Square::e8) => true,
//             //
//             (Square::f1, Square::f1) => true,
//             (Square::f2, Square::f2) => true,
//             (Square::f3, Square::f3) => true,
//             (Square::f4, Square::f4) => true,
//             (Square::f5, Square::f5) => true,
//             (Square::f6, Square::f6) => true,
//             (Square::f7, Square::f7) => true,
//             (Square::f8, Square::f8) => true,
//             //
//             (Square::g1, Square::g1) => true,
//             (Square::g2, Square::g2) => true,
//             (Square::g3, Square::g3) => true,
//             (Square::g4, Square::g4) => true,
//             (Square::g5, Square::g5) => true,
//             (Square::g6, Square::g6) => true,
//             (Square::g7, Square::g7) => true,
//             (Square::g8, Square::g8) => true,
//             //
//             (Square::h1, Square::h1) => true,
//             (Square::h2, Square::h2) => true,
//             (Square::h3, Square::h3) => true,
//             (Square::h4, Square::h4) => true,
//             (Square::h5, Square::h5) => true,
//             (Square::h6, Square::h6) => true,
//             (Square::h7, Square::h7) => true,
//             (Square::h8, Square::h8) => true,
//             _ => false,
//         }
//     }
// }
