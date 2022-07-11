library errors;

pub enum InitError {
    AlreadyInitialized: (),
    TokenSupplyCannotBeZero: (),
}

pub enum AccessError {
    SenderNotPermittedToMint: (),
}
