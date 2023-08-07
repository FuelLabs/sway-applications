library;

/// Error log for when transaction execution panics.
pub enum ExecutionError {
    /// Emitted when the called `target` is not a [Identity::ContractId].
    CanOnlyCallContracts: (),
    /// Emitted when the recovered addresses in `count_approvals `are not in ascending order (0x1 < 0x2 < 0x3...) [b256].
    IncorrectSignerOrdering: (),
    /// Emitted when the amount of the asset being sent is greater than the balance in the contract.
    InsufficientAssetAmount: (),
    /// Emitted when the total approval count is less than the required threshold for execution.
    InsufficientApprovals: (),
    /// Emitted when attempting to transfer with `transfer_params.value` as [Option::None].
    TransferRequiresAValue: (),
}

/// Error log for when transaction execution panics.
pub enum InitError {
    /// Emitted when calling the constructor it has already been called.
    CannotReinitialize: (),
    /// Emitted when the constructor has not been called to initialize the contract yet.
    NotInitialized: (),
    /// Emitted when When `THRESHOLD` is zero [u64].
    ThresholdCannotBeZero: (),
    /// Emitted when `THRESHOLD` is greater the sum of the weights from users in `users` [Vec<User>].
    TotalWeightCannotBeLessThanThreshold: (),
}
