library data_structures;

pub enum State {
    Initialize: (),
    Uninitialize: (),
}

impl core::ops::Eq for State {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (State::Initialize, State::Initialize) => true,
            (State::Uninitialize, State::Uninitialize) => true,
            _ => false,
        }
    }
}

pub struct NFTMetadata {
    // This is left as an example. Support for dynamic length string is needed here
    name: str[7],
}

impl NFTMetadata {
    fn new() -> Self {
        Self {
            name: "Example",
        }
    }
}
