library;

/// Errors related to the arbiter parameters.
pub enum ArbiterInputError {
    /// The asset sent is not the same as the accepted asset for the fee.
    AssetDoesNotMatch: (),
    /// The arbiter cannot be the buyer.
    CannotBeBuyer: (),
    /// The arbiter cannot be the seller.
    CannotBeSeller: (),
    /// The arbiter cannot have zero fee.
    FeeCannotBeZero: (),
    /// An amount other than the required fee was sent.
    FeeDoesNotMatchAmountSent: (),
    /// Payment cannot be larger than the fee amount.
    PaymentTooLarge: (),
}

/// Errors related to the asset parameters.
pub enum AssetInputError {
    /// No assets were specified.
    UnspecifiedAssets: (),
    /// The asset amount cannot be zero.
    AssetAmountCannotBeZero: (),
}

/// Errors related to the deadline.
pub enum DeadlineInputError {
    /// The deadline cannot be in the past.
    MustBeInTheFuture: (),
}

/// Errors related to deposits.
pub enum DepositError {
    /// Incorrect amount of asset was sent.
    IncorrectAssetAmount: (),
    /// Incorrect assets were sent.
    IncorrectAssetSent: (),
}

/// Errors related to the state of the escrow.
pub enum StateError {
    /// The user has already deposited.
    AlreadyDeposited: (),
    /// The escrow has already been disputed.
    AlreadyDisputed: (),
    /// The arbiter has not been proposed.
    ArbiterHasNotBeenProposed: (),
    /// Cannot dispute an escrow before depositing the required assets.
    CannotDisputeBeforeDeposit: (),
    /// Cannot resolve an escrow before depositing the required assets.
    CannotResolveBeforeDeposit: (),
    /// Cannot take payment before the deadline.
    CannotTakePaymentBeforeDeadline: (),
    /// Cannot take payment during a dispute.
    CannotTakePaymentDuringDispute: (),
    /// Cannot transfer if the assets were never deposited.
    CannotTransferBeforeDeposit: (),
    /// The buyer cannot withdraw after depositing.
    CannotWithdrawAfterDeposit: (),
    /// Cannot withdraw before the deadline.
    CannotWithdrawBeforeDeadline: (),
    /// The escrow has expired.
    EscrowExpired: (),
    /// The escrow has not been disputed.
    NotDisputed: (),
    /// The escrow has already been completed.
    StateNotPending: (),
}

/// Errors made by users.
pub enum UserError {
    /// The user is not authorized to perform the action.
    Unauthorized: (),
}

/// Errors related to the user input.
pub enum UserInputError {
    /// The specified recipient is invalid.
    InvalidRecipient: (),
}
