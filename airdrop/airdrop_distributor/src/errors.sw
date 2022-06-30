library errors;

pub enum AccessError {
    AirdropDoesNotExist: (),
    SenderNotAdmin: (),
    UserAlreadyClaimed: (),
}

pub enum InitError {
    AirdropAmountCannotBeZero: (),
    ClaimTimeCannotBeZero: (),
    IncorrectTokenContract: (),
}

pub enum StateError {
    AlreadyInitalized: (),
    ClaimPeriodHasEnded: (),
    ClaimPeriodHasNotEnded: (),
    NotInitalized: (),
}

pub enum VerificationError {
    MerkleProofFailed: (),
}
