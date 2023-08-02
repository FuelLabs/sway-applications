library;

pub enum AccessError {
    CallerNotAdmin: (),
    NotEnoughTokens: (),
    UserAlreadyClaimed: (),
}

pub enum InitError {
    AlreadyInitialized: (),
    CannotAirdropZeroTokens: (),
}

pub enum StateError {
    ClaimPeriodNotActive: (),
    ClaimPeriodActive: (),
}

pub enum VerificationError {
    MerkleProofFailed: (),
}
