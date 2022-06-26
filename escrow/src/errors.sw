library errors;

pub enum AccessError {
    UnauthorizedUser: (),
}

pub enum ApproveError {
    AlreadyApproved: (),
}

pub enum CreationError {
    ArbitorFeeCannotExceed100Percent: (),
    BuyerCannotBeArbitor: (),
    DeadlineMustBeInTheFuture: (),
    DepositAmountCannotBeZero: (),
    SellerCannotBeArbitor: (),
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
