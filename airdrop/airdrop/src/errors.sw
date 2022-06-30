library errors;

pub enum AccessError {
    UserAlreadyClaimed: (),
}

pub enum StateError {
    AlreadyInitalized: (),
    NotInitalized: (),
}

pub enum VerificationError {
    MerkleProofFailed: (),
}
