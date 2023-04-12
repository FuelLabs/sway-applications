library;

pub enum AccessControlError {
    AuthorizationError: (),
}

pub enum FundingError {
    InsufficientContractBalance: u64,
}

pub enum TransactionError {
    DuplicateTransaction: b256,
    InvalidTransaction: b256,
    /// (start_timestamp, end_timestamp, your_timestamp)
    /// Valid for: start_timestamp <= your_timestamp <= end_timestamp
    TimestampNotInRange: (u64, u64, u64),
}
