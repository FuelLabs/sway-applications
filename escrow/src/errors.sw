library errors;

pub enum AccessError {
    InvalidIdentifier: (),
    UnauthorizedUser: (),
}

pub enum ApproveError {
    AlreadyApproved: (),
}

pub enum CreationError {
    AssetAmountCannotBeZero: (),
    AssetIdCannotBeZero: (),
}

pub enum DepositError {
    AlreadyDeposited: (),
    DepositRequired: (),
    IncorrectAssetAmount: (),
    IncorrectAssetDeposited: (),
}

pub enum InitError {
    CannotReinitialize: (),
}

pub enum StateError {
    StateNotInitialized: (),
    StateNotPending: (),
}
