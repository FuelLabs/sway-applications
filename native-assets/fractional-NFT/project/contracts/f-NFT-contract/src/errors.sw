library;

/// Error thrown when there is an issue with the Vault SubId.
pub enum SubIdError {
    /// Logged when an invalid SubId is passed as an argument.
    InvalidSubId: (),
}

/// Error thrown when something goes wrong with the deposit function.
pub enum DepositError {
    /// Logged when the asset sent does not adhere to the SRC-20 standard.
    InvalidSRC20NFT: (),
}

/// Error thrown when something goes wrong with the withdraw function.
pub enum WithdrawError {
    /// Logged when not all shares have been included in the transaction.
    AllSharesNotReturned: (),
    /// Logged when the asset sent is not valid shares of an NFT.
    InvalidAsset: (),
}
