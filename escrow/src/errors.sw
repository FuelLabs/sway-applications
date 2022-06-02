library errors;

pub enum Error {
    ApproveError: ApproveError,
    DepositError: DepositError,
    InitError: InitError,
    StateError: StateError,
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
