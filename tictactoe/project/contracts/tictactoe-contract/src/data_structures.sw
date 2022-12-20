library data_structures;

pub enum GameState {
    InProgress: (),
    PlayerOneWon: (),
    PlayerTwoWon: (),
    Draw: (),
}

impl core::ops::Eq for GameState {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (GameState::InProgress, GameState::InProgress) => true,
            (GameState::PlayerOneWon, GameState::PlayerOneWon) => true,
            (GameState::PlayerTwoWon, GameState::PlayerTwoWon) => true,
            (GameState::Draw, GameState::Draw) => true,
            _ => false,
        }
    }
}
