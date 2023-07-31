library;

pub enum ExecutionError {
    CallingFunctionsRequiresCalldata: (),
    CallingFunctionsRequiresSingleValueTypeArg: (),
    CanOnlyCallContracts: (),
    IncorrectSignerOrdering: (),
    InsufficientAssetAmount: (),
    InsufficientApprovals: (),
    TransferRequiresAnAssetId: (),
    TransferRequiresAValue: (),
}

pub enum InitError {
    CannotReinitialize: (),
    NotInitialized: (),
    ThresholdCannotBeZero: (),
    TotalWeightCannotBeLessThanThreshold: (),
}
