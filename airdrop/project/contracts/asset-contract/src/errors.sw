library;

pub enum AccessError {
    SenderNotPermittedToMint: (),
}

pub enum InitError {
    AlreadyInitialized: (),
    AssetSupplyCannotBeZero: (),
}

pub enum InputError {
    GreaterThanMaximumSupply: (),
}
