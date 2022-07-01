library errors;

pub enum CreationError {
    DurationCannotBeZero: (),
    InvalidAcceptancePercentage: (),
}

pub enum InitializationError {
    CannotReinitialize: (),
    ContractNotInitialized: (),
}

pub enum ProposalError {
    InsufficientApprovals: (),
    ProposalExpired: (),
    ProposalStillActive: (),
}

pub enum UserError {
    AmountCannotBeZero: (),
    IncorrectAssetSent: (),
    InsufficientBalance: (),
    InvalidId: (),
    VoteAmountCannotBeZero: (),
}
