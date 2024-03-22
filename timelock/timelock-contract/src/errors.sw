library;

/// Errors related to access control.
pub enum AccessControlError {
    /// The user is not authorized to perform the action.
    AuthorizationError: (),
}

/// Errors related to balances.
pub enum FundingError {
    /// The user does not have enough balance to perform the action.
    InsufficientContractBalance: u64,
}

/// Errors related to the transaction.
pub enum TransactionError {
    /// The transaction is a duplicate.
    DuplicateTransaction: b256,
    /// The transaction is invalid.
    InvalidTransaction: b256,
    /// The timestamp is not in the valid range.
    /// Valid range is: start_timestamp <= your_timestamp <= end_timestamp
    /// The order of the values is (start_timestamp, end_timestamp, your_timestamp).
    TimestampNotInRange: (u64, u64, u64),
}

impl AbiEncode for TransactionError {
    fn abi_encode(self, ref mut buffer: Buffer) {
        match self {
            Self::DuplicateTransaction(value) => {
                buffer.push(value);
            },
            Self::InvalidTransaction(value) => {
                buffer.push(value);
            },
            Self::TimestampNotInRange(value) => {
                buffer.push(value);
            }
        }
    }
}
