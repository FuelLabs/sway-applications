library;

/// Errors related to permissions.
pub enum AccessError {
    /// The sender is not permitted to mint tokens.
    SenderNotPermittedToMint: (),
}

/// Errors related to the initialization of the contract.
pub enum InitError {
    /// The contract has already been initialized.
    AlreadyInitialized: (),
    /// The maximum supply of the token must be greater than zero.
    AssetSupplyCannotBeZero: (),
}

/// Errors related to input parameters.
pub enum InputError {
    /// The input amount is greater than the maximum supply.
    GreaterThanMaximumSupply: (),
}
