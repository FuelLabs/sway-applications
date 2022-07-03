library errors;

pub enum CreationError {
    ArbiterFeeCannotExceed100Percent: (),
    ArbiterCannotBeBuyer: (),
    ArbiterCannotBeSeller: (),
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

pub enum UserError {
    AlreadyDisputed: (),
    CannotDisputeBeforeDesposit: (),
    CannotTakePaymentBeforeDeadline: (),
    CannotTakePaymentDuringDispute: (),
    CannotTransferBeforeDesposit: (),
    CannotTransferPaymentDuringDispute: (),
    CannotResolveBeforeDesposit: (),
    InvalidRecipient: (),
    NotDisputed: (),
    UnauthorizedUser: (),
}
