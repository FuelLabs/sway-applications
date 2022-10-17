library errors;

pub enum InitError {
    CannotReinitialize: (),
    NotInitialized: (),
    IdenticalAssets: (),
}

pub enum InputError {
    SentInvalidAmount: (),
    SentInvalidAsset: (),
}

pub enum TransactionError {
    CannotSatisfyConstraint: (),
    DeadlinePassed: (),
    InsufficientDeposit: (),
    InsufficientLiquidity: (),
    InsufficientReserve: (),
}
