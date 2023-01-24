library errors;

pub enum AccessControlError {
    CanOnlyBeAccessedByAnOwner: (),
}

pub enum ExecutionError {
    IncorrectSignerOrdering: (),
    InsufficientAssetAmount: (),
    InsufficientApprovals: (),
}

pub enum InitError {
    AddressCannotBeZero: (),
    CannotReinitialize: (),
    NotInitialized: (),
    ThresholdCannotBeZero: (),
    TotalWeightCannotBeLessThanThreshold: (),
    WeightingCannotBeZero: (),
}
