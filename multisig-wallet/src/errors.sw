library errors;

pub enum Error {
    ExecutionError: ExecutionError,
    InitError: InitError,
}

pub enum InitError {
    AddressCannotBeZero: (),
    CannotReinitialize: (),
    NotInitialized: (),
    ThresholdCannotBeZero: (),
    WeightingCannotBeZero: (),
}

pub enum ExecutionError {
    IncorrectSignerOrdering: (),
    InsufficientAssetAmount: (),
    InsufficientApprovals: (),
}
