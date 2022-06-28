library errors;

pub enum AccessError {
    UnauthorizedUser: (),
}

pub enum ApproveError {
    AlreadyApproved: (),
}

pub enum CreationError {
    ArbitorFeeCannotExceed100Percent: (),
    ArbitorCannotBeBuyer: (),
    ArbitorCannotBeSeller: (),
    DeadlineMustBeInTheFuture: (),
    DepositAmountCannotBeZero: (),
    UnspecifiedAssets: (),
    UnspecifiedBuyers: (),
}

pub enum DepositError {
    AlreadyDeposited: (),
    DepositRequired: (),
    IncorrectAssetAmount: (),
    IncorrectAssetDeposited: (),
}

pub enum StateError {
    StateNotPending: (),
}
