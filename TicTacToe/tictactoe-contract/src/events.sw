library;

/// Event for when a game results in a draw.
pub struct GameDrawnEvent {
    /// The first player.
    pub player_one: Identity,
    /// The second player.
    pub player_two: Identity,
}

/// Event for when a game is won.
pub struct GameWonEvent {
    /// The winning player.
    pub player: Identity,
}

/// Event for when a new game is started.
pub struct NewGameEvent {
    /// The first player.
    pub player_one: Identity,
    /// The second player.
    pub player_two: Identity,
}
