library errors;

pub enum InitError {
    AssetPairAlreadySet: (),
    AssetPairNotSet: (),
    IdenticalAssets: (),
}

pub enum InputError {
    CannotAddLessThanMinimumLiquidity: u64,
    DeadlinePassed: u64,
    ExpectedNonZeroAmount: ContractId,
    ExpectedNonZeroParameter: ContractId,
    InvalidAsset: (),
}

pub enum TransactionError {
    DesiredAmountTooHigh: u64,
    DesiredAmountTooLow: u64,
    ExpectedNonZeroDeposit: ContractId,
    InsufficientReserve: ContractId,
    NoLiquidityToRemove: (),
}
