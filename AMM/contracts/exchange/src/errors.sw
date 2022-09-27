library errors;

pub enum TransactionError {
    DeadlineHasPassed: (),
    InsufficientInput: (),
    InsufficientLiquidity: (),
    InsufficientReserve: (),
    SenderDoesNotHaveEnoughBalance: (),
}

pub enum InputError {
    MessageAmountCannotBeZero: (),
    MessageAmountShouldBeZero: (),
    MessageAssetIdDoesNotMatch: (),
    PassedAmountCannotBeZero: (),
    PassedAssetIdDoesNotMatch: (),
}
