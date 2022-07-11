library errors;

pub enum AccessError {
    SenderNotPermittedToMint: (),
}

pub enum InitError {
    AlreadyInitialized: (),
    TokenSupplyCannotBeZero: (),
}

pub enum InputError {
    GreaterThanMaximumSupply: (),
}
