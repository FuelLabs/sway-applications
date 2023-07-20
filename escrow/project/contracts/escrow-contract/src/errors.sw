library;

pub enum ArbiterInputError {
    AssetDoesNotMatch: (),
    CannotBeBuyer: (),
    CannotBeSeller: (),
    FeeCannotBeZero: (),
    FeeDoesNotMatchAmountSent: (),
    PaymentTooLarge: (),
}

pub enum AssetInputError {
    UnspecifiedAssets: (),
    AssetAmountCannotBeZero: (),
}

pub enum DeadlineInputError {
    MustBeInTheFuture: (),
}

pub enum DepositError {
    IncorrectAssetAmount: (),
    IncorrectAssetSent: (),
}

pub enum StateError {
    AlreadyDeposited: (),
    AlreadyDisputed: (),
    ArbiterHasNotBeenProposed: (),
    CannotDisputeBeforeDeposit: (),
    CannotResolveBeforeDeposit: (),
    CannotTakePaymentBeforeDeadline: (),
    CannotTakePaymentDuringDispute: (),
    CannotTransferBeforeDeposit: (),
    CannotWithdrawAfterDeposit: (),
    CannotWithdrawBeforeDeadline: (),
    EscrowExpired: (),
    NotDisputed: (),
    StateNotPending: (),
}

pub enum UserError {
    Unauthorized: (),
}

pub enum UserInputError {
    InvalidRecipient: (),
}
