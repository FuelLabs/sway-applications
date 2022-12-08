library data_structures;

pub enum State {
    Initialized: (),
    Uninitialized: (),
}

impl core::ops::Eq for State {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (State::Initialized, State::Initialized) => true,
            (State::Uninitialized, State::Uninitialized) => true,
            _ => false,
        }
    }
}

pub struct TokenMetadata {
    // This is left as an example. Support for dynamic length string is needed here
    name: str[7],
}

impl TokenMetadata {
    pub fn new() -> Self {
        Self {
            name: "Example",
        }
    }
}
