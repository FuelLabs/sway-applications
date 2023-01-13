library move;

dep square;
dep piece;

use square::Square;
use piece::Piece;

pub struct Move {
    source: Square,
    dest: Square,
    promotion: Option<Piece>,
}

impl Move {
    pub fn build(src: Square, dest: Square, promotion: Option<Piece>) -> Move {
        Move {
            source: src,
            dest: dest,
            promotion: promotion,
        }
    }
}


//////////////////////////////////////////////////////////////////
/// TESTS
//////////////////////////////////////////////////////////////////
#[test()]
fn test_move_builder() {
    let sq_1 = Square::a2;
    let sq_2 = Square::a3;
    let my_move = Move::build(sq_1, sq_2, Option::None);
    assert(my_move.source.to_index() == sq_1.to_index());
    assert(my_move.dest.to_index() == sq_2.to_index());
    let promo = if let Option::None = my_move.promotion {
        false
    } else {
        true
    };
    assert(promo == false);
}
