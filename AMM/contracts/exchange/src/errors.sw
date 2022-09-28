library errors;

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
