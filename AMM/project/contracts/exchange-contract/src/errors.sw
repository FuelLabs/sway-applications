library;

/// Determines the type of error during initialisation.
///
/// ### Variants
///
/// * `AssetPairAlreadySet`: () - The asset pair has already been set.
/// * `AssetPairNotSet`: `()` - The asset pair has not been set.
/// * `IdenticalAssets`: () - The input and output assets are the same.
pub enum InitError {
    AssetPairAlreadySet: (),
    AssetPairNotSet: (),
    IdenticalAssets: (),
}

/// Determines the type of error regarding inputs.
///
/// ### Variants
///
/// * `CannotAddLessThanMinimumLiquidity`: u64 - The amount of liquidity added is less than the minimum amount.
/// * `DeadlinePassed`: `u64` - The deadline has passed.
/// * `ExpectedNonZeroAmount`: ContractId - The input amount was not greater than zero.
/// * `ExpectedNonZeroParameter`: ContractId - The parameter was not greater than zero.
/// * `InvalidAsset`: () - The provided asset id is invalid.
pub enum InputError {
    CannotAddLessThanMinimumLiquidity: u64,
    DeadlinePassed: u64,
    ExpectedNonZeroAmount: ContractId,
    ExpectedNonZeroParameter: ContractId,
    InvalidAsset: (),
}

/// Determines the type of error regarding transactions.
///
/// ### Variants
///
/// * `DesiredAmountTooHigh`: u64 - The desired amount is too high.
/// * `DesiredAmountTooLow`: `u64` - The desired amount is too low.
/// * `ExpectedNonZeroDeposit`: ContractId - The deposit amount was not greater than zero.
/// * `InsufficientReserve`: ContractId - The reserve amount is too low.
/// * `NoLiquidityToRemove`: () - The total liquidity is not greater than zero .
pub enum TransactionError {
    DesiredAmountTooHigh: u64,
    DesiredAmountTooLow: u64,
    ExpectedNonZeroDeposit: ContractId,
    InsufficientReserve: ContractId,
    NoLiquidityToRemove: (),
}
