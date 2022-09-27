library errors;

// ANCHOR: error
pub enum MoveError {
    OccupiedSquare: (),
    OutOfBounds: (),
}
// ANCHOR_END: error
