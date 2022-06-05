library errors;

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
