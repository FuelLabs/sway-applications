library errors;

pub enum AccessError {
    AirdropDoesNotExist: (),
    UserAlreadyClaimed: (),
}

pub enum InitError {
    AirdropAmountCannotBeZero: (),
    ClaimTimeCannotBeZero: (),
    IncorrectTokenContract: (),
}

pub enum StateError {
    AlreadyInitalized: (),
    NotInitalized: (),
}

pub enum VerificationError {
    MerkleProofFailed: (),
}
