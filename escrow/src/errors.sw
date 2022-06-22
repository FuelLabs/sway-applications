library errors;

pub enum AccessError {
    InvalidIdentifier: (),
    UnauthorizedUser: (),
}

pub enum ApproveError {
    AlreadyApproved: (),
}

pub enum CreationError {
    DepositAmountCannotBeZero: (),
}

pub enum DepositError {
    AlreadyDeposited: (),
    DepositRequired: (),
    IncorrectAssetAmount: (),
    IncorrectAssetDeposited: (),
}

pub enum InitError {
    CannotReinitialize: (),
    NotInitialized: (),
    OwnerAddressCannotBeZero: (),
}

pub enum StateError {
    StateNotInitialized: (),
    StateNotPending: (),
}
