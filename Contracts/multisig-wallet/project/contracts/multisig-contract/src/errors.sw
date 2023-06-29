library;

pub enum ExecutionError {
    IncorrectSignerOrdering: (),
    InsufficientAssetAmount: (),
    InsufficientApprovals: (),
}

pub enum InitError {
    CannotReinitialize: (),
    NotInitialized: (),
    ThresholdCannotBeZero: (),
    TotalWeightCannotBeLessThanThreshold: (),
}
