library errors;

pub enum InitError {
    CannotReinitialize: (),
    NotInitialized: (),
    PoolAssetsCannotBeIdentical: (),
}

pub enum InputError {
    AmountCannotBeZero: (),
    AmountMustBeZero: (),
    AmountTooLow: u64,
    DeadlinePassed: u64,
    InvalidAsset: (),
}

pub enum TransactionError {
    DepositCannotBeZero: (),
    DesiredAmountTooHigh: u64,
    InsufficientLiquidity: (),
    LiquidityCannotBeZero: (),
    ProvidedAmountTooLow: u64,
}
