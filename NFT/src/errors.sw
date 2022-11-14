library errors;

pub enum AccessError {
    MaxTokensMinted: (),
    SenderNotAdmin: (),
}

pub enum InitError {
    CannotReinitialized: (),
    NotInitialized: (),
}
