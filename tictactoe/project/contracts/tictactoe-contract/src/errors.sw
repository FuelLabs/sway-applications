library errors;

pub enum Error {
    NotYourTurn: (),
    GameIsOver: (),
    PositionAlreadyTaken: (),
    InvalidPosition: (),
}
