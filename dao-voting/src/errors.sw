library errors;

pub enum InitializationError {
    CannotReinitialize: (),
    ContractNotInitialized: (),
}

pub enum CreationError {
    AcceptancePercentageCannotBeAboveOneHundred: (),
    AcceptancePercentageCannotBeZero: (),
    EndHeightCannotBeZero: (),
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
