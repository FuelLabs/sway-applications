library errors;

pub enum InitError {
    AssetPairAlreadySet: (),
    AssetPairNotSet: (),
    IdenticalAssets: (),
}

pub enum InputError {
    CannotAddLessThanMinimumLiquidity: u64,
    DeadlinePassed: u64,
    ExpectedNonZeroAmount: (),
    ExpectedNonZeroParameter: (),
    ExpectedZeroAmount: (),
    InvalidAsset: (),
}

pub enum TransactionError {
    DesiredAmountTooHigh: u64,
    ExpectedNonZeroDeposit: (),
    InsufficientReserve: (),
    NoLiquidityToRemove: (),
}
