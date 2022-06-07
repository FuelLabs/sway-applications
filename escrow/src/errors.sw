library errors;

pub enum AccessError {
    UnauthorizedUser: (),
}

pub enum ApproveError {
    AlreadyApproved: (),
}

pub enum DepositError {
    AlreadyDeposited: (),
    DepositRequired: (),
    IncorrectAssetAmount: (),
    IncorrectAssetDeposited: (),
}

pub enum InitError {
    AssetAmountCannotBeZero: (),
    AssetIdCannotBeZero: (),
    CannotReinitialize: (),
}

pub enum StateError {
    StateNotInitialized: (),
    StateNotPending: (),
}
