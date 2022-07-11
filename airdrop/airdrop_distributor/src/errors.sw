library errors;

pub enum AccessError {
    UserAlreadyClaimed: (),
}

pub enum InitError {
    ClaimTimeCannotBeZero: (),
    AlreadyInitalized: (),
}

pub enum StateError {
    ClaimPeriodHasEnded: (),
}

pub enum VerificationError {
    MerkleProofFailed: (),
}
