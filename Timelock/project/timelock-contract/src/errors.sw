library errors;

pub enum AccessControlError {
    AuthorizationError: (),
}

pub enum FundingError {
    InsufficientContractBalance: u64,
}

pub enum TransactionError {
    DuplicateTransaction: b256,
    InvalidTransaction: b256,
    TimestampNotInRange: (u64, u64, u64),
}
