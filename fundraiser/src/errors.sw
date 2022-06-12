library errors;

pub enum CreationError {
    CannotUseNativeAsset: (),
    ContractNotInitialized: (),
    DeadlineMustBeInTheFuture: (),
    TargetAmountCannotBeZero: (),
}

pub enum UserError {
    AlreadyClaimed: (),
    FundraiseEnded: (),
    FundraiseNotSuccessful: (),
    IncorrectAssetSent: (),
    NoSuchCampaign: (),
    UnauthorizedUser: (),
}
