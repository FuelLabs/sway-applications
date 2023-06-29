library;

pub enum GameStateError {
    GameHasEnded: (),
    GameHasNotEnded: (),
}

pub enum PlayerError {
    IncorrectPlayerTurn: (),
}

pub enum PositionError {
    CellIsNotEmpty: (),
    InvalidPosition: (),
}
