library;

/// Errors related to the state of the game.
pub enum GameStateError {
    /// The game has ended.
    GameHasEnded: (),
    /// The game has not ended yet.
    GameHasNotEnded: (),
}

/// Errors made by players.
pub enum PlayerError {
    /// It is not the player's turn.
    IncorrectPlayerTurn: (),
}

/// Errors related to the position of a cell.
pub enum PositionError {
    /// The cell is already occupied.
    CellIsNotEmpty: (),
    /// The cell is out of bounds.
    InvalidPosition: (),
}
