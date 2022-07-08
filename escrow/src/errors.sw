library errors;

// TODO

pub enum CreationError {
    ArbiterCannotBeBuyer: (),
    ArbiterCannotBeSeller: (),
    ArbiterFeeCannotBeZero: (),
    ArbiterFeeDoesNotMatchAmountSent: (),
    DeadlineMustBeInTheFuture: (),
    DepositAmountCannotBeZero: (),
    UnspecifiedAssets: (),
    UnspecifiedBuyers: (),
}

pub enum DepositError {
    AlreadyDeposited: (),
    DepositRequired: (),
    EscrowExpired: (),
    IncorrectAssetAmount: (),
    IncorrectAssetDeposited: (),
}

pub enum StateError {
    StateNotPending: (),
}

pub enum UserError {
    AlreadyDisputed: (),
    ArbiterPaymentCannotBeGreaterThanDepositFromSeller: (),
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
