library errors;

pub enum AccessControlError {
    AuthorizationError: (),
}

pub enum TransactionError {
    DuplicateTransaction: b256,
    IncorrectAmountSent: (u64, u64),
    InsufficientContractBalance: u64,
    InvalidTransaction: b256,
    TimestampNotInRange: (u64, u64, u64),
}
