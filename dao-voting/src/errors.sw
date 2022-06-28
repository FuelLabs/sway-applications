library errors;

pub enum CreationError {
    AcceptancePercentageCannotBeAboveOneHundred: (),
    AcceptancePercentageCannotBeZero: (),
    DeadlineCannotBeZero: (),
}

pub enum InitializationError {
    CannotReinitialize: (),
    ContractNotInitialized: (),
}

pub enum ProposalError {
    ApprovalPercentageNotMet: (),
    ProposalExpired: (),
    ProposalStillActive: (),
}

pub enum UserError {
    AmountCannotBeZero: (),
    IncorrectAssetSent: (),
    InvalidId: (),
    NotEnoughAssets: (),
    VoteAmountCannotBeZero: (),
}
