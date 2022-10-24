library errors;

pub enum InitError {
    CannotReinitialize: (),
    IdenticalAssets: (),
    NotInitialized: (),
}

pub enum InputError {
    AmountCannotBeZero: (),
    AmountMustBeZero: (),
    AmountTooLow: u64,
    DeadlinePassed: (),
    InvalidAsset: (),
}

pub enum TransactionError {
    DepositCannotBeZero: (),
    DesiredAmountTooHigh: u64,
    InsufficientLiquidity: (),
    LiquidityCannotBeZero: (),
    ProvidedAmountTooLow: u64,
}
