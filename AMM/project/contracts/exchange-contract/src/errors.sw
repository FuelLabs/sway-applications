library;

/// Determines the type of error during initialisation.
pub enum InitError {
    /// The asset pair has already been set.
    AssetPairAlreadySet: (),
    /// The asset pair has not been set.
    AssetPairNotSet: (),
    /// The input and output assets are the same.
    IdenticalAssets: (),
}

/// Determines the type of error regarding inputs.
pub enum InputError {
    /// The amount of liquidity added is less than the minimum amount.
    CannotAddLessThanMinimumLiquidity: u64,
    /// The deadline has passed.
    DeadlinePassed: u64,
    /// The input amount was not greater than zero.
    ExpectedNonZeroAmount: AssetId,
    /// The parameter was not greater than zero.
    ExpectedNonZeroParameter: AssetId,
    /// The provided asset id is invalid.
    InvalidAsset: (),
}

/// Determines the type of error regarding transactions.
pub enum TransactionError {
    /// The desired amount is too high.
    DesiredAmountTooHigh: u64,
    /// The desired amount is too low.
    DesiredAmountTooLow: u64,
    /// The deposit amount was not greater than zero.
    ExpectedNonZeroDeposit: AssetId,
    /// The reserve amount is too low.
    InsufficientReserve: AssetId,
    /// The total liquidity is not greater than zero.
    NoLiquidityToRemove: (),
}
