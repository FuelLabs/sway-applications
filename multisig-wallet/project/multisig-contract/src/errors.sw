library errors;

pub enum AccessControlError {
    CanOnlyBeAccessedByAnOwner: (),
}

pub enum ExecutionError {
    CallingFunctionsRequiresCalldata: (),
    CallingFunctionsRequiresSingleValueTypeArg: (),
    CannotCallFunctionsOnAddresses: (),
    IncorrectSignerOrdering: (),
    InsufficientAssetAmount: (),
    InsufficientApprovals: (),
    TransferRequiresAnAssetId: (),
    TransferRequiresAValue: (),
}

pub enum InitError {
    AddressCannotBeZero: (),
    CannotReinitialize: (),
    NotInitialized: (),
    ThresholdCannotBeZero: (),
    TotalWeightCannotBeLessThanThreshold: (),
    WeightingCannotBeZero: (),
}
