library errors;

pub enum MoveError {
    OccupiedSquare: (),
    OutOfBounds: (),
}
