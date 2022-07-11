library errors;

pub enum AccessError {
    UserAlreadyClaimed: (),
}

pub enum InitError {
    AlreadyInitialized: (),
    ClaimTimeCannotBeZero: (),
}

pub enum StateError {
    ClaimPeriodHasEnded: (),
}

pub enum VerificationError {
    MerkleProofFailed: (),
}
