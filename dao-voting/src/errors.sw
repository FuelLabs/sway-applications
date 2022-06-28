library errors;

pub enum CreationError {
    InvalidAcceptancePercentage: (),
    DeadlineCannotBeZero: (),
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
    InvalidId: (),
    InsuffiecientBalance: (),
    VoteAmountCannotBeZero: (),
}
