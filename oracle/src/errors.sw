library errors;

pub enum AccessError {
    NotOwner: (),
}

pub enum InitializationError {
    CannotReinitialize: (),
    ContractNotInitialized: (),
}
