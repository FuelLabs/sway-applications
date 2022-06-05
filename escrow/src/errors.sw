library errors;

pub enum AccessError {
    UnauthorizedUser: (),
}

pub enum InitError {
    AssetAmountCannotBeZero: (),
    AssetIdCannotBeZero: (),
    CannotReinitialize: (),
}

pub enum DepositError {
    AlreadyDeposited: (),
    DepositRequired: (),
    IncorrectAssetAmount: (),
    IncorrectAssetDeposited: (),
}

pub enum ApproveError {
    AlreadyApproved: (),
}

pub enum StateError {
    StateNotInitialized: (),
    StateNotPending: (),
}
