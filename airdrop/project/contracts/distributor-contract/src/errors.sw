library;

/// Errors related to permissions.
pub enum AccessError {
    /// The caller is not the admin of the contract.
    CallerNotAdmin: (),
    /// There are not enough tokens in the contract to perform the operation.
    NotEnoughTokens: (),
    /// The user has already claimed their tokens.
    UserAlreadyClaimed: (),
}

/// Errors related to the initialization of the contract.
pub enum InitError {
    /// The contract has already been initialized.
    AlreadyInitialized: (),
    /// No assets were transferred during initialization.
    CannotAirdropZeroTokens: (),
}

/// Errors related to the state of the contract.
pub enum StateError {
    /// The claim period is not active.
    ClaimPeriodNotActive: (),
    /// The claim period is active.
    ClaimPeriodActive: (),
}

/// Errors related to the verification of the merkle proof.
pub enum VerificationError {
    /// The merkle proof verification failed.
    MerkleProofFailed: (),
}
