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
    CannotReinitialize: (),
    NotInitialized: (),
    ThresholdCannotBeZero: (),
    TotalWeightCannotBeLessThanThreshold: (),
<<<<<<< HEAD:multisig-wallet/project/multisig-contract/src/errors.sw
    WeightingCannotBeZero: (),
=======
>>>>>>> origin/master:multisig-wallet/project/contracts/multisig-contract/src/errors.sw
}
