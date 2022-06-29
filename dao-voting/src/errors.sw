library errors;

pub enum CreationError {
    DeadlineCannotBeZero: (),
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
