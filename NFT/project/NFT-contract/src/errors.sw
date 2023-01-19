library errors;

pub enum AccessError {
    MaxTokensMinted: (),
    NoContractAdmin: (),
    SenderNotAdmin: (),
}

pub enum InitError {
    CannotReinitialize: (),
    NotInitialized: (),
}
