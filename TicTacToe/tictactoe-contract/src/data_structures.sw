library;

use core::ops::Eq;

/// Represents the state of a game.
pub enum State {
    /// The game is currently being played.
    Playing: (),
    /// The game has ended.
    Ended: (),
}

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (State::Playing, State::Playing) => true,
            (State::Ended, State::Ended) => true,
            _ => false,
        }
    }
}
