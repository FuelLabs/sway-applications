library data_structures;

pub struct TokenMetaData {
    // This is left as an example. Support for dynamic length string is needed here
    name: str[7],
}

impl TokenMetaData {
    fn new() -> Self {
        Self {
            name: "Example",
        }
    }
}
