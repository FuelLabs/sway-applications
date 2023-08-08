library;

/// Error log for when transaction execution reverts.
pub enum ExecutionError {
    /// Emitted when the called `target` is not a [Identity::ContractId].
    CanOnlyCallContracts: (),
    /// Emitted when the recovered addresses in `count_approvals` are not in ascending order (0x1 < 0x2 < 0x3...) [b256].
    IncorrectSignerOrdering: (),
    /// Emitted when the amount of the asset being sent is greater than the balance in the contract.
    InsufficientAssetAmount: (),
    /// Emitted when the total approval count is less than the required threshold for execution.
    InsufficientApprovals: (),
    /// Emitted when attempting to transfer with `transfer_params.value` as [Option::None].
    TransferRequiresAValue: (),
}

/// Error log for when transaction execution reverts.
pub enum InitError {
    /// Emitted when calling the constructor more than once.
    CannotReinitialize: (),
    /// Emitted when the constructor has not been called.
    NotInitialized: (),
    /// Emitted when the `THRESHOLD` is zero [u64].
    ThresholdCannotBeZero: (),
    /// Emitted when `THRESHOLD` is greater than the sum of the weights from `users` [Vec<User>].
    TotalWeightCannotBeLessThanThreshold: (),
}
